use crate::ippresponse::attributes::{JobAttribute, PrinterAttribute};
use crate::printer::{IppVersion, Job, Printer};
use ipp::proto::attribute::IppAttribute;
use ipp::proto::model::{DelimiterTag, StatusCode};
use ipp::proto::request::IppRequestResponse;
use ipp::proto::value::IppValue;
use std::convert::Infallible;

pub struct IppResponseBuilder {
    version: IppVersion,
    status: StatusCode,
    operational_attributes: Vec<IppAttribute>,
    unsupported_attributes: Vec<IppAttribute>,
    printer_attributes: Vec<IppAttribute>,
    job_attributes: Vec<IppAttribute>,
    request_id: u32,
}

impl IppResponseBuilder {
    pub fn new(status: StatusCode, request_id: u32) -> Self {
        Self {
            version: IppVersion::V1_1,
            status,
            operational_attributes: Vec::new(),
            unsupported_attributes: Vec::new(),
            printer_attributes: Vec::new(),
            job_attributes: Vec::new(),
            request_id,
        }
    }

    pub fn add_job_attribute(&mut self, job: &Job, attribute: JobAttribute) {
        let attr = job.protofy_attribute(attribute).unwrap();
        self.job_attributes.push(attr);
    }

    pub fn add_required_job_attributes(&mut self, job: &Job) {
        // IPP/1.1 Attributes
        self.add_job_attribute(job, JobAttribute::JobId);
        self.add_job_attribute(job, JobAttribute::JobUri);
        self.add_job_attribute(job, JobAttribute::JobState);
        self.add_job_attribute(job, JobAttribute::JobStateReasons);
    }

    pub fn add_printer_attribute(&mut self, printer: &Printer, attr: PrinterAttribute) {
        let attr = printer.protofy_attribute(attr).unwrap();
        self.printer_attributes.push(attr);
    }

    pub fn add_required_printer_attributes(&mut self, printer: &Printer) {
        // IPP/1.1 Attributes
        self.add_printer_attribute(printer, PrinterAttribute::CharsetConfigured);
        self.add_printer_attribute(printer, PrinterAttribute::CharsetSupported);
        self.add_printer_attribute(printer, PrinterAttribute::CompressionSupported);
        self.add_printer_attribute(printer, PrinterAttribute::DocumentFormatDefault);
        self.add_printer_attribute(printer, PrinterAttribute::DocumentFormatSupported);
        self.add_printer_attribute(printer, PrinterAttribute::GeneratedNaturalLanguageSupported);
        self.add_printer_attribute(printer, PrinterAttribute::IppVersionsSupported);
        self.add_printer_attribute(printer, PrinterAttribute::NaturalLanguageConfigured);
        self.add_printer_attribute(printer, PrinterAttribute::OperationsSupported);
        self.add_printer_attribute(printer, PrinterAttribute::PdlOverrideSupported);
        self.add_printer_attribute(printer, PrinterAttribute::PrinterIsAcceptingJobs);
        self.add_printer_attribute(printer, PrinterAttribute::PrinterName);
        self.add_printer_attribute(printer, PrinterAttribute::PrinterState);
        self.add_printer_attribute(printer, PrinterAttribute::PrinterStateReasons);
        self.add_printer_attribute(printer, PrinterAttribute::PrinterUpTime);
        self.add_printer_attribute(printer, PrinterAttribute::PrinterUriSupported);
        self.add_printer_attribute(printer, PrinterAttribute::QueuedJobCount);
        self.add_printer_attribute(printer, PrinterAttribute::UriAuthenticationSupported);
        self.add_printer_attribute(printer, PrinterAttribute::UriSecuritySupported);
    }

    pub fn build(&self) -> Result<IppRequestResponse, Infallible> {
        // Create the response
        let mut resp = IppRequestResponse::new_response(
            ipp::proto::model::IppVersion::from(self.version),
            self.status,
            self.request_id,
        );

        for attr in &self.operational_attributes {
            resp.attributes_mut()
                .add(DelimiterTag::OperationAttributes, attr.clone());
        }

        for attr in &self.unsupported_attributes {
            resp.attributes_mut()
                .add(DelimiterTag::UnsupportedAttributes, attr.clone());
        }

        for attr in &self.printer_attributes {
            resp.attributes_mut()
                .add(DelimiterTag::PrinterAttributes, attr.clone());
        }

        for attr in &self.job_attributes {
            resp.attributes_mut()
                .add(DelimiterTag::JobAttributes, attr.clone());
        }

        Ok(resp)
    }
}

impl Printer {
    fn protofy_attribute(&self, attribute: PrinterAttribute) -> Result<IppAttribute, String> {
        match attribute {
            PrinterAttribute::CharsetConfigured => Ok(IppAttribute::new(
                "charset-configured",
                IppValue::Charset(String::from(self.charset_configured)),
            )),
            PrinterAttribute::CharsetSupported => {
                let mut charsets = Vec::<IppValue>::new();
                for &charset in &self.charset_supported {
                    charsets.push(IppValue::Charset(String::from(charset)));
                }
                Ok(IppAttribute::new(
                    "charset-supported",
                    IppValue::Array(charsets),
                ))
            }
            PrinterAttribute::CompressionSupported => {
                let mut compressions = Vec::<IppValue>::new();
                for &compression in &self.compression_supported {
                    compressions.push(IppValue::Keyword(String::from(compression)))
                }
                Ok(IppAttribute::new(
                    "compression-supported",
                    IppValue::Array(compressions),
                ))
            }
            PrinterAttribute::DocumentFormatDefault => Ok(IppAttribute::new(
                "document-format-default",
                IppValue::MimeMediaType(String::from(self.document_format_default)),
            )),
            PrinterAttribute::DocumentFormatSupported => {
                let mut formats = Vec::<IppValue>::new();
                for &format in &self.document_format_supported {
                    formats.push(IppValue::MimeMediaType(String::from(format)));
                }
                Ok(IppAttribute::new(
                    "document-format-supported",
                    IppValue::Array(formats),
                ))
            }
            PrinterAttribute::GeneratedNaturalLanguageSupported => {
                let mut languages = Vec::<IppValue>::new();
                for &lang in &self.generated_natural_language_supported {
                    languages.push(IppValue::NaturalLanguage(String::from(lang)));
                }
                Ok(IppAttribute::new(
                    "generated-natural-language-supported",
                    IppValue::Array(languages),
                ))
            }
            PrinterAttribute::IppVersionsSupported => {
                let mut versions = Vec::<IppValue>::new();
                for &ver in &self.ipp_versions_supported {
                    versions.push(IppValue::Keyword(String::from(ver)));
                }
                Ok(IppAttribute::new(
                    "ipp-versions-supported",
                    IppValue::Array(versions),
                ))
            }
            PrinterAttribute::NaturalLanguageConfigured => Ok(IppAttribute::new(
                "natural-language-configured",
                IppValue::NaturalLanguage(String::from(self.natural_language_configured)),
            )),
            PrinterAttribute::OperationsSupported => {
                let mut ops = Vec::<IppValue>::new();
                for &op in &self.operations_supported {
                    ops.push(IppValue::Enum(op as i32))
                }
                Ok(IppAttribute::new(
                    "operations-supported",
                    IppValue::Array(ops),
                ))
            }
            PrinterAttribute::PdlOverrideSupported => Ok(IppAttribute::new(
                "pdl-override-supported",
                IppValue::Keyword(String::from(self.pdl_override_supported)),
            )),
            PrinterAttribute::PrinterIsAcceptingJobs => Ok(IppAttribute::new(
                "printer-is-accepting-jobs",
                IppValue::Boolean(self.printer_is_accepting_jobs),
            )),
            PrinterAttribute::PrinterName => Ok(IppAttribute::new(
                "printer-name",
                IppValue::NameWithoutLanguage(self.printer_name.clone()),
            )),
            PrinterAttribute::PrinterState => Ok(IppAttribute::new(
                "printer-state",
                IppValue::Enum(self.printer_state as i32),
            )),
            PrinterAttribute::PrinterStateReasons => {
                let mut reasons = Vec::<IppValue>::new();
                for &reason in &self.printer_state_reasons {
                    reasons.push(IppValue::Keyword(String::from(reason)));
                }
                Ok(IppAttribute::new(
                    "printer-state-reasons",
                    IppValue::Array(reasons),
                ))
            }
            PrinterAttribute::PrinterUpTime => Ok(IppAttribute::new(
                "printer-up-time",
                IppValue::Integer(self.printer_up_time as i32),
            )),
            PrinterAttribute::PrinterUriSupported => {
                let mut uris = Vec::<IppValue>::new();
                for uri in &self.printer_uri_supported {
                    uris.push(IppValue::Uri(uri.uri.clone()));
                }
                Ok(IppAttribute::new(
                    "printer-uri-supported",
                    IppValue::Array(uris),
                ))
            }
            PrinterAttribute::QueuedJobCount => Ok(IppAttribute::new(
                "queued-job-count",
                IppValue::Integer(self.queued_job_count() as i32),
            )),
            PrinterAttribute::UriAuthenticationSupported => {
                let mut auth_methods = Vec::<IppValue>::new();
                for uri in &self.printer_uri_supported {
                    auth_methods.push(IppValue::Keyword(String::from(uri.authentication)));
                }
                Ok(IppAttribute::new(
                    "uri-authentication-supported",
                    IppValue::Array(auth_methods),
                ))
            }
            PrinterAttribute::UriSecuritySupported => {
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

impl Job {
    fn protofy_attribute(&self, attribute: JobAttribute) -> Result<IppAttribute, String> {
        match attribute {
            JobAttribute::JobId => Ok(IppAttribute::new(
                "job-id",
                IppValue::Integer(self.id as i32),
            )),
            JobAttribute::JobUri => Ok(IppAttribute::new(
                "job-uri",
                IppValue::Uri(self.uri.clone()),
            )),
            JobAttribute::JobState => Ok(IppAttribute::new(
                "job-state",
                IppValue::Enum(self.state as i32),
            )),
            JobAttribute::JobStateReasons => {
                let mut reasons = Vec::<IppValue>::new();
                for &reason in &self.state_reasons {
                    reasons.push(IppValue::Keyword(String::from(reason)));
                }
                Ok(IppAttribute::new(
                    "job-state-reasons",
                    IppValue::Array(reasons),
                ))
            }
        }
    }
}
