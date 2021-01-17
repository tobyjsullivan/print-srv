use std::convert::Infallible;
use std::net::SocketAddr;

use futures::future::BoxFuture;
use futures::FutureExt;
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use ipp::prelude::*;
use ipp::proto::parser::IppParser;
use ipp::proto::reader::IppReader;
use log::{info, trace, warn};
use num_traits::FromPrimitive;

use crate::ippresponse::IppResponseBuilder;
use printer::Printer;

mod ippresponse;
mod printer;

async fn print_ipp_request(req: &mut IppRequestResponse) {
    println!("IPP Request:");
    let header = req.header();
    println!("OperationStatus (Raw): {:?}", header.operation_status);
    let operation = Operation::from_u16(header.operation_status);
    println!("OperationStatus: {:?}", operation);
    println!("Request ID: {}", header.request_id);
    let version = parse_version(header.version).unwrap();
    println!("Version: {}", version);

    for attr in req.attributes().groups() {
        println!("Attribute Group:");
        println!("Tag: {:?}", attr.tag());
        for entry in attr.attributes() {
            println!("Attribute: {} = {:?}", entry.0, entry.1);
        }
    }

    println!("Payload:");
    // Read the payload in full
    // Note: this consumes the payload from the request. You won't be able to read it again.
    let mut writer = Vec::<u8>::new();
    async_std::io::copy(req.payload_mut(), &mut writer)
        .await
        .unwrap();
    println!("{:?}", writer);
}

fn parse_version(v: IppVersion) -> Result<printer::IppVersion, ()> {
    if v.0 == IppVersion::v1_0().0 {
        Ok(printer::IppVersion::V1_0)
    } else if v.0 == IppVersion::v1_1().0 {
        Ok(printer::IppVersion::V1_1)
    } else if v.0 == IppVersion::v2_0().0 {
        Ok(printer::IppVersion::V2_0)
    } else if v.0 == IppVersion::v2_1().0 {
        Ok(printer::IppVersion::V2_1)
    } else if v.0 == IppVersion::v2_2().0 {
        Ok(printer::IppVersion::V2_2)
    } else {
        Err(())
    }
}

fn print_ipp_response(req: &IppRequestResponse) {
    println!("IPP Request:");
    let header = req.header();
    println!("OperationStatus (Raw): {:?}", header.operation_status);
    let status = StatusCode::from_u16(header.operation_status);
    println!("OperationStatus: {:?}", status);
    println!("Request ID: {}", header.request_id);
    let version = parse_version(header.version).unwrap();
    println!("Version: {}", version);

    for attr in req.attributes().groups() {
        println!("Attribute Group:");
        println!("Tag: {:?}", attr.tag());
        for entry in attr.attributes() {
            println!("Attribute: {} = {:?}", entry.0, entry.1);
        }
    }
}

async fn handle(printer: &Printer, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("--- Request Received ---");
    let method = req.method().as_str();
    let path = req.uri().path();
    let mut cont = false;
    println!("{} {}", method, path);
    for hdr in req.headers() {
        let name = hdr.0.as_str();
        let val = hdr.1.to_str().unwrap();

        if name.to_lowercase() == "expect" && val.to_lowercase() == "100-continue" {
            cont = true;
        }

        println!("{}: {}", name, val);
    }
    if req.method() != Method::POST || req.uri().path() != "/ipp/print" {
        println!("Returning: 404 Not Found");
        return Ok(Response::builder()
            .status(404)
            .body(Body::from("404 Not Found\n"))
            .unwrap());
    }

    let (_parts, body) = req.into_parts();
    let bytes = body::to_bytes(body).await.unwrap();
    let parser = IppParser::new(IppReader::new(futures::io::Cursor::new(bytes)));
    let resp_body = match parser.parse().await {
        Ok(mut req) => {
            let resp = handle_ipp(printer, &mut req).await;
            Body::from(resp.to_bytes().to_vec())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Body::from("Hello World")
        }
    };
    let status = if cont { 100 } else { 200 };
    println!("Response Status: {}", status);
    Ok(Response::builder().status(200).body(resp_body).unwrap())
}

impl Printer {}

async fn handle_ipp(printer: &Printer, req: &mut IppRequestResponse) -> IppRequestResponse {
    print_ipp_request(req).await;

    let operation: Operation = Operation::from_u16(req.header().operation_status).unwrap();
    let response: BoxFuture<Result<IppRequestResponse, Infallible>> =
        if operation == Operation::GetPrinterAttributes {
            handle_get_printer_attributes(printer, req).boxed()
        } else if operation == Operation::ValidateJob {
            handle_validate_job(printer, req).boxed()
        } else {
            async {
                let header = req.header();
                let builder = IppResponseBuilder::new(
                    printer,
                    StatusCode::ServerErrorOperationNotSupported,
                    header.request_id,
                );
                Ok(builder.build().unwrap())
            }
            .boxed()
        };

    let resp: IppRequestResponse = response.await.unwrap();

    println!("Sending Response:");
    print_ipp_response(&resp);
    resp
}

async fn handle_get_printer_attributes(
    printer: &Printer,
    req: &IppRequestResponse,
) -> Result<IppRequestResponse, Infallible> {
    // Find the OperationAttributes attribute group(s)
    let mut requested_attr_keywords: Vec<String> = vec![];
    for group in req
        .attributes()
        .groups_of(DelimiterTag::OperationAttributes)
    {
        // Find the list of requested attributes
        match group.attributes().get("requested-attributes") {
            Some(attr) => match attr.value() {
                IppValue::Array(values) => {
                    for keyword_value in values {
                        match keyword_value {
                            IppValue::Keyword(keyword) => {
                                requested_attr_keywords.push(keyword.clone())
                            }
                            _ => warn!(
                                "Found unexpected value type in requested-attributes: {:?}",
                                keyword_value
                            ),
                        }
                    }
                }
                attr => warn!(
                    "Found unexpected value attribute value for requested-attributes: {:?}",
                    attr
                ),
            },
            None => warn!("Did not find requested-attributes in OperationAttributes group."),
        }
    }

    println!(
        "The client has requested these attributes: {}",
        requested_attr_keywords.join(",")
    );

    let header = req.header();
    let mut builder = IppResponseBuilder::new(printer, StatusCode::SuccessfulOK, header.request_id);

    builder.add_required_printer_attributes();

    Ok(builder.build().unwrap())
}

// https://tools.ietf.org/html/rfc8011#section-4.2.3
async fn handle_validate_job(
    printer: &Printer,
    req: &IppRequestResponse,
) -> Result<IppRequestResponse, Infallible> {
    let header = req.header();
    let mut builder = IppResponseBuilder::new(printer, StatusCode::SuccessfulOK, header.request_id);

    // TODO: Return same Operation Attributes and Unsupported Attributes as Print-Job operation (but no Job Attributes)
    // https://tools.ietf.org/html/rfc8011#section-4.2.3

    Ok(builder.build().unwrap())
}

async fn ipp_response(req: &IppRequestResponse) -> Result<IppRequestResponse, Infallible> {
    let header = req.header();
    let mut resp = IppRequestResponse::new_response(
        header.version,
        StatusCode::SuccessfulOK,
        header.request_id,
    );

    // let attr = IppAttribute::new("")
    // resp.attributes_mut()

    Ok(resp)
}

#[tokio::main]
async fn main() {
    let printer = Printer::default();

    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Create a MakeService to handle each connection...
    let make_service = make_service_fn(move |_conn| {
        let printer = printer.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let printer = printer.clone();
                async move { handle(&printer, req).await }
            }))
        }
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
