use std::convert::Infallible;
use std::net::SocketAddr;

use async_std::prelude::*;
use futures::{Future, FutureExt};
use futures::future::BoxFuture;
use hyper::{Body, Method, Request, Response, Server};
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use ipp::prelude::*;
use ipp::proto::parser::IppParser;
use ipp::proto::reader::IppReader;
use log::{info, trace, warn};
use num_traits::FromPrimitive;
use tokio::macros::support::{Pin, Poll};
use tokio::task;

use printer::Printer;
use crate::printer::Charset;

mod printer;

fn print_ipp_request(req: &IppRequestResponse) {
    println!("IPP Request:");
    let header = req.header();
    println!("OperationStatus (Raw): {:?}", header.operation_status);
    let operation = Operation::from_u16(header.operation_status);
    println!("OperationStatus: {:?}", operation);
    println!("Request ID: {}", header.request_id);
    println!("Version: {}", header.version.0);

    for attr in req.attributes().groups() {
        println!("Attribute Group:");
        println!("Tag: {:?}", attr.tag());
        for entry in attr.attributes() {
            println!("Attribute: {} = {:?}", entry.0, entry.1);
        }
    }
}

fn print_ipp_response(req: &IppRequestResponse) {
    println!("IPP Request:");
    let header = req.header();
    println!("OperationStatus (Raw): {:?}", header.operation_status);
    let status = StatusCode::from_u16(header.operation_status);
    println!("OperationStatus: {:?}", status);
    println!("Request ID: {}", header.request_id);
    println!("Version: {}", header.version.0);

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
        return Ok(Response::builder().status(404).body(Body::from("404 Not Found\n")).unwrap());
    }

    let (parts, body) = req.into_parts();
    let bytes = body::to_bytes(body).await.unwrap();
    let parser = IppParser::new(IppReader::new(futures::io::Cursor::new(bytes)));
    let resp_body = match parser.parse().await {
        Ok(req) => {
            let resp = handle_ipp(printer, &req).await;
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

async fn handle_ipp(printer: &Printer, req: &IppRequestResponse) -> IppRequestResponse {
    print_ipp_request(&req);

    let operation: Operation = Operation::from_u16(req.header().operation_status).unwrap();
    let response: BoxFuture<Result<IppRequestResponse, Infallible>> =
        if operation == Operation::GetPrinterAttributes {
            handle_get_printer_attributes(printer, req).boxed()
        } else {
            ipp_response(req).boxed()
        };

    let resp: IppRequestResponse = response.await.unwrap();

    println!("Sending Response:");
    print_ipp_response(&resp);
    resp
}

async fn handle_get_printer_attributes(printer: &Printer, req: &IppRequestResponse) -> Result<IppRequestResponse, Infallible> {
    // Find the OperationAttributes attribute group(s)
    let mut requestedAttrKeywords: Vec<String> = vec![];
    for group in req.attributes().groups_of(DelimiterTag::OperationAttributes) {
        // Find the list of requested attributes
        match group.attributes().get("requested-attributes") {
            Some(attr) => {
                match attr.value() {
                    IppValue::Array(values) => {
                        for keywordValue in values {
                            match keywordValue {
                                IppValue::Keyword(keyword) => requestedAttrKeywords.push(keyword.clone()),
                                _ => warn!("Found unexpected value type in requested-attributes: {:?}", keywordValue),
                            }
                        }
                    }
                    attr => warn!("Found unexpected value attribute value for requested-attributes: {:?}", attr),
                }
            }
            None => warn!("Did not find requested-attributes in OperationAttributes group."),
        }
    }

    println!("The client has requested these attributes: {}", requestedAttrKeywords.join(","));

    // Create the response
    let header = req.header();
    let mut resp = IppRequestResponse::new_response(
        header.version,
        StatusCode::SuccessfulOK,
        header.request_id,
    );

    // EXPECT charset-configured
    add_attribute(printer, &mut resp, Attribute::Printer(PrinterAttribute::CharsetConfigured));
    // EXPECT charset-supported
    add_attribute(printer, &mut resp, Attribute::Printer(PrinterAttribute::CharsetSupported));
    // EXPECT compression-supported
    add_attribute(printer, &mut resp, Attribute::Printer(PrinterAttribute::CompressionSupported));
    // EXPECT document-format-default
    add_attribute(printer, &mut resp, Attribute::Printer(PrinterAttribute::DocumentFormatDefault));
    // EXPECT document-format-supported
    add_attribute(printer, &mut resp, Attribute::Printer(PrinterAttribute::DocumentFormatSupported));
    // EXPECT generated-natural-language-supported
    // EXPECT ipp-versions-supported
    // EXPECT natural-language-configured
    // EXPECT operations-supported
    // EXPECT pdl-override-supported
    // EXPECT printer-is-accepting-jobs
    // EXPECT printer-name
    // EXPECT printer-state
    // EXPECT printer-state-reasons
    // EXPECT printer-up-time
    // EXPECT printer-uri-supported
    // EXPECT queued-job-count
    // EXPECT uri-authentication-supported
    // EXPECT uri-security-supported

    Ok(resp)
}

fn add_attribute(printer: &Printer, res: &mut IppRequestResponse, attr: Attribute) {
    let delim = attr.get_delimiter_tag();
    if let Ok(printer_attr) = printer.get_attribute(attr) {
        res.attributes_mut().add(delim, printer_attr);
    }
}

enum Attribute {
    Printer(PrinterAttribute),
}

enum PrinterAttribute {
    CharsetConfigured,
    CharsetSupported,
    CompressionSupported,
    DocumentFormatDefault,
    DocumentFormatSupported,
}

impl Attribute {
    fn get_delimiter_tag(&self) -> DelimiterTag {
        match self {
            Attribute::Printer(_) => DelimiterTag::PrinterAttributes,
        }
    }
}

impl Printer {
    fn get_attribute(&self, attribute: Attribute) -> Result<IppAttribute, String> {
        match attribute {
            Attribute::Printer(PrinterAttribute::CharsetConfigured) => {
                Ok(IppAttribute::new(
                    "charset-configured",
                    IppValue::Charset(self.charset_configured.into()),
                ))
            }
            Attribute::Printer(PrinterAttribute::CharsetSupported) => {
                let mut charsets = Vec::<IppValue>::new();
                for &charset in &self.charset_supported {
                    charsets.push(IppValue::Charset(charset.into()));
                }
                Ok(IppAttribute::new(
                    "charset-supported",
                    IppValue::Array(charsets),
                ))
            }
            Attribute::Printer(PrinterAttribute::CompressionSupported) => {
                let mut compressions = Vec::<IppValue>::new();
                for &compression in &self.compression_supported {
                    compressions.push(IppValue::Keyword(compression.into()))
                }
                Ok(IppAttribute::new(
                    "compression-supported",
                    IppValue::Array(compressions),
                ))
            }
            Attribute::Printer(PrinterAttribute::DocumentFormatDefault) => {
                Ok(IppAttribute::new(
                    "document-format-default",
                    IppValue::MimeMediaType(self.document_format_default.into()),
                ))
            }
            Attribute::Printer(PrinterAttribute::DocumentFormatSupported) => {
                let mut formats = Vec::<IppValue>::new();
                for &format in &self.document_format_supported {
                    formats.push(IppValue::MimeMediaType(format.into()));
                }
                Ok(IppAttribute::new(
                    "document-format-supported",
                    IppValue::Array(formats),
                ))
            }
        }
    }
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
                async move {
                    handle(&printer, req).await
                }
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