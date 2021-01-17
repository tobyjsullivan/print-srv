use crate::ippresponse::attributes::{Attribute, PrinterAttribute};
use crate::printer::{IppVersion, Printer};
use ipp::proto::attribute::IppAttribute;
use ipp::proto::model::{DelimiterTag, StatusCode};
use ipp::proto::request::IppRequestResponse;
use ipp::proto::value::IppValue;
use std::convert::Infallible;

pub struct IppResponseBuilder<'a> {
    printer: &'a Printer,
    version: IppVersion,
    status: StatusCode,
    attributes: Vec<Attribute>,
    request_id: u32,
}

impl<'a> IppResponseBuilder<'a> {
    pub fn new(printer: &'a Printer, status: StatusCode, request_id: u32) -> Self {
        Self {
            printer,
            version: IppVersion::V1_1,
            status,
            attributes: Vec::new(),
            request_id,
        }
    }

    pub fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr);
    }

    pub fn add_required_printer_attributes(&mut self) {
        // IPP/1.1 Attributes
        self.add_attribute(Attribute::Printer(PrinterAttribute::CharsetConfigured));
        self.add_attribute(Attribute::Printer(PrinterAttribute::CharsetSupported));
        self.add_attribute(Attribute::Printer(PrinterAttribute::CompressionSupported));
        self.add_attribute(Attribute::Printer(PrinterAttribute::DocumentFormatDefault));
        self.add_attribute(Attribute::Printer(
            PrinterAttribute::DocumentFormatSupported,
        ));
        self.add_attribute(Attribute::Printer(
            PrinterAttribute::GeneratedNaturalLanguageSupported,
        ));
        self.add_attribute(Attribute::Printer(PrinterAttribute::IppVersionsSupported));
        self.add_attribute(Attribute::Printer(
            PrinterAttribute::NaturalLanguageConfigured,
        ));
        self.add_attribute(Attribute::Printer(PrinterAttribute::OperationsSupported));
        self.add_attribute(Attribute::Printer(PrinterAttribute::PdlOverrideSupported));
        self.add_attribute(Attribute::Printer(PrinterAttribute::PrinterIsAcceptingJobs));
        self.add_attribute(Attribute::Printer(PrinterAttribute::PrinterName));
        self.add_attribute(Attribute::Printer(PrinterAttribute::PrinterState));
        self.add_attribute(Attribute::Printer(PrinterAttribute::PrinterStateReasons));
        self.add_attribute(Attribute::Printer(PrinterAttribute::PrinterUpTime));
        self.add_attribute(Attribute::Printer(PrinterAttribute::PrinterUriSupported));
        self.add_attribute(Attribute::Printer(PrinterAttribute::QueuedJobCount));
        self.add_attribute(Attribute::Printer(
            PrinterAttribute::UriAuthenticationSupported,
        ));
        self.add_attribute(Attribute::Printer(PrinterAttribute::UriSecuritySupported));
    }

    pub fn build(&self) -> Result<IppRequestResponse, Infallible> {
        // Create the response
        let mut resp = IppRequestResponse::new_response(
            ipp::proto::model::IppVersion::from(self.version),
            self.status,
            self.request_id,
        );

        for &attr in &self.attributes {
            add_attribute(self.printer, &mut resp, attr);
        }

        Ok(resp)
    }
}

fn add_attribute(printer: &Printer, res: &mut IppRequestResponse, attr: Attribute) {
    let delim = attr.get_delimiter_tag();
    if let Ok(printer_attr) = printer.protofy_attribute(attr) {
        res.attributes_mut().add(delim, printer_attr);
    }
}

impl Attribute {
    pub fn get_delimiter_tag(&self) -> DelimiterTag {
        match self {
            Attribute::Printer(_) => DelimiterTag::PrinterAttributes,
        }
    }
}

impl Printer {
    fn protofy_attribute(&self, attribute: Attribute) -> Result<IppAttribute, String> {
        match attribute {
            Attribute::Printer(PrinterAttribute::CharsetConfigured) => Ok(IppAttribute::new(
                "charset-configured",
                IppValue::Charset(String::from(self.charset_configured)),
            )),
            Attribute::Printer(PrinterAttribute::CharsetSupported) => {
                let mut charsets = Vec::<IppValue>::new();
                for &charset in &self.charset_supported {
                    charsets.push(IppValue::Charset(String::from(charset)));
                }
                Ok(IppAttribute::new(
                    "charset-supported",
                    IppValue::Array(charsets),
                ))
            }
            Attribute::Printer(PrinterAttribute::CompressionSupported) => {
                let mut compressions = Vec::<IppValue>::new();
                for &compression in &self.compression_supported {
                    compressions.push(IppValue::Keyword(String::from(compression)))
                }
                Ok(IppAttribute::new(
                    "compression-supported",
                    IppValue::Array(compressions),
                ))
            }
            Attribute::Printer(PrinterAttribute::DocumentFormatDefault) => Ok(IppAttribute::new(
                "document-format-default",
                IppValue::MimeMediaType(String::from(self.document_format_default)),
            )),
            Attribute::Printer(PrinterAttribute::DocumentFormatSupported) => {
                let mut formats = Vec::<IppValue>::new();
                for &format in &self.document_format_supported {
                    formats.push(IppValue::MimeMediaType(String::from(format)));
                }
                Ok(IppAttribute::new(
                    "document-format-supported",
                    IppValue::Array(formats),
                ))
            }
            Attribute::Printer(PrinterAttribute::GeneratedNaturalLanguageSupported) => {
                let mut languages = Vec::<IppValue>::new();
                for &lang in &self.generated_natural_language_supported {
                    languages.push(IppValue::NaturalLanguage(String::from(lang)));
                }
                Ok(IppAttribute::new(
                    "generated-natural-language-supported",
                    IppValue::Array(languages),
                ))
            }
            Attribute::Printer(PrinterAttribute::IppVersionsSupported) => {
                let mut versions = Vec::<IppValue>::new();
                for &ver in &self.ipp_versions_supported {
                    versions.push(IppValue::Keyword(String::from(ver)));
                }
                Ok(IppAttribute::new(
                    "ipp-versions-supported",
                    IppValue::Array(versions),
                ))
            }
            Attribute::Printer(PrinterAttribute::NaturalLanguageConfigured) => {
                Ok(IppAttribute::new(
                    "natural-language-configured",
                    IppValue::NaturalLanguage(String::from(self.natural_language_configured)),
                ))
            }
            Attribute::Printer(PrinterAttribute::OperationsSupported) => {
                let mut ops = Vec::<IppValue>::new();
                for &op in &self.operations_supported {
                    ops.push(IppValue::Enum(op as i32))
                }
                Ok(IppAttribute::new(
                    "operations-supported",
                    IppValue::Array(ops),
                ))
            }
            Attribute::Printer(PrinterAttribute::PdlOverrideSupported) => Ok(IppAttribute::new(
                "pdl-override-supported",
                IppValue::Keyword(String::from(self.pdl_override_supported)),
            )),
            Attribute::Printer(PrinterAttribute::PrinterIsAcceptingJobs) => Ok(IppAttribute::new(
                "printer-is-accepting-jobs",
                IppValue::Boolean(self.printer_is_accepting_jobs),
            )),
            Attribute::Printer(PrinterAttribute::PrinterName) => Ok(IppAttribute::new(
                "printer-name",
                IppValue::NameWithoutLanguage(self.printer_name.clone()),
            )),
            Attribute::Printer(PrinterAttribute::PrinterState) => Ok(IppAttribute::new(
                "printer-state",
                IppValue::Enum(self.printer_state as i32),
            )),
            Attribute::Printer(PrinterAttribute::PrinterStateReasons) => {
                let mut reasons = Vec::<IppValue>::new();
                for &reason in &self.printer_state_reasons {
                    reasons.push(IppValue::Keyword(String::from(reason)));
                }
                Ok(IppAttribute::new(
                    "printer-state-reasons",
                    IppValue::Array(reasons),
                ))
            }
            Attribute::Printer(PrinterAttribute::PrinterUpTime) => Ok(IppAttribute::new(
                "printer-up-time",
                IppValue::Integer(self.printer_up_time as i32),
            )),
            Attribute::Printer(PrinterAttribute::PrinterUriSupported) => {
                let mut uris = Vec::<IppValue>::new();
                for uri in &self.printer_uri_supported {
                    uris.push(IppValue::Uri(uri.uri.clone()));
                }
                Ok(IppAttribute::new(
                    "printer-uri-supported",
                    IppValue::Array(uris),
                ))
            }
            Attribute::Printer(PrinterAttribute::QueuedJobCount) => Ok(IppAttribute::new(
                "queued-job-count",
                IppValue::Integer(self.queued_job_count() as i32),
            )),
            Attribute::Printer(PrinterAttribute::UriAuthenticationSupported) => {
                let mut auth_methods = Vec::<IppValue>::new();
                for uri in &self.printer_uri_supported {
                    auth_methods.push(IppValue::Keyword(String::from(uri.authentication)));
                }
                Ok(IppAttribute::new(
                    "uri-authentication-supported",
                    IppValue::Array(auth_methods),
                ))
            }
            Attribute::Printer(PrinterAttribute::UriSecuritySupported) => {
                let mut sec_methods = Vec::<IppValue>::new();
                for uri in &self.printer_uri_supported {
                    sec_methods.push(IppValue::Keyword(String::from(uri.security)));
                }
                Ok(IppAttribute::new(
                    "uri-security-supported",
                    IppValue::Array(sec_methods),
                ))
            }
        }
    }
}
