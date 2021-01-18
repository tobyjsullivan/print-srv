pub use crate::printer::charset::Charset;
pub use crate::printer::compression::Compression;
pub use crate::printer::ippversion::IppVersion;
pub use crate::printer::job::Job;
pub use crate::printer::mimemediatype::MimeMediaType;
pub use crate::printer::naturallanguage::NaturalLanguage;
pub use crate::printer::operation::Operation;
pub use crate::printer::pdloverride::PdlOverride;
pub use crate::printer::printerstate::PrinterState;
use crate::printer::printerstate::{PrinterStateReason, PrinterStateReasonKeyword};
use crate::printer::uri::{PrinterUri, UriAuthenticationMethod, UriSecurityMethod};

mod charset;
mod compression;
mod ippversion;
mod job;
mod jobstate;
mod mimemediatype;
mod naturallanguage;
mod operation;
mod pdloverride;
mod printerstate;
mod uri;

#[derive(Debug)]
pub struct Printer {
    pub charset_configured: Charset,
    pub charset_supported: Vec<Charset>,
    pub compression_supported: Vec<Compression>,
    pub document_format_default: MimeMediaType,
    pub document_format_supported: Vec<MimeMediaType>,
    pub generated_natural_language_supported: Vec<NaturalLanguage>,
    pub ipp_versions_supported: Vec<IppVersion>,
    pub natural_language_configured: NaturalLanguage,
    pub operations_supported: Vec<Operation>,
    pub pdl_override_supported: PdlOverride,
    pub printer_is_accepting_jobs: bool,
    pub printer_name: String,
    pub printer_state: PrinterState,
    pub printer_state_reasons: Vec<PrinterStateReason>,
    pub printer_up_time: u32,
    pub printer_uri_supported: Vec<PrinterUri>,
    pub jobs: Vec<Job>,
    next_job_id: u32,
}

impl Default for Printer {
    fn default() -> Self {
        Self {
            charset_configured: Charset::Utf8,
            charset_supported: vec![Charset::Utf8],
            compression_supported: vec![Compression::None],
            document_format_default: MimeMediaType::Pdf,
            document_format_supported: vec![MimeMediaType::Pdf, MimeMediaType::PlainText],
            generated_natural_language_supported: vec![NaturalLanguage::EN],
            ipp_versions_supported: vec![IppVersion::V1_1],
            natural_language_configured: NaturalLanguage::EN,
            operations_supported: vec![Operation::PrintJob],
            pdl_override_supported: PdlOverride::Attempted,
            printer_is_accepting_jobs: true,
            printer_name: String::from("Default Printer Name"),
            printer_state: PrinterState::Idle,
            printer_state_reasons: vec![PrinterStateReason {
                keyword: PrinterStateReasonKeyword::None,
                severity: None,
            }],
            printer_up_time: 1, // TODO: Must increment every second. https://tools.ietf.org/html/rfc8011#section-5.4.29
            printer_uri_supported: vec![PrinterUri::new(
                "ipp://127.0.0.1:3000/ipp/print",
                UriAuthenticationMethod::None,
                UriSecurityMethod::None,
            )],
            jobs: Vec::new(),
            next_job_id: 1,
        }
    }
}

impl Printer {
    pub fn queued_job_count(&self) -> u32 {
        // TODO: Count pending or processing jobs
        // https://tools.ietf.org/html/rfc8011#section-5.4.24
        return 0;
    }

    pub fn new_job(&mut self, data: &[u8]) -> Job {
        let job_id = self.next_job_id;
        self.next_job_id += 1;
        let printer_uri = self.printer_uri_supported.get(0).unwrap().uri.clone();
        let job_uri = format!("{}/{}", printer_uri, job_id);
        let job = Job::new(job_id, job_uri, data);
        self.jobs.push(job.clone()); // TODO: Refactor so that we're not cloning Jobs
        job
    }
}
