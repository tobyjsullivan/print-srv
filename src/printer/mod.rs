use std::convert::TryFrom;

pub use crate::printer::charset::Charset;
pub use crate::printer::compression::Compression;
pub use crate::printer::ippversion::IppVersion;
pub use crate::printer::mimemediatype::MimeMediaType;
pub use crate::printer::naturallanguage::NaturalLanguage;
pub use crate::printer::operation::Operation;
pub use crate::printer::pdloverride::PdlOverride;
pub use crate::printer::printerstate::PrinterState;
use crate::printer::printerstate::{PrinterStateReason, PrinterStateReasonKeyword};

mod charset;
mod compression;
mod mimemediatype;
mod naturallanguage;
mod ippversion;
mod operation;
mod pdloverride;
mod printerstate;

#[derive(Clone, Debug)]
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
    // 	EXPECT printer-up-time
    // 	EXPECT printer-uri-supported
    // 	EXPECT queued-job-count
    // 	EXPECT uri-authentication-supported
    // 	EXPECT uri-security-supported
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
            ipp_versions_supported: vec![IppVersion::V1_1, IppVersion::V2_0, IppVersion::V2_1, IppVersion::V2_2],
            natural_language_configured: NaturalLanguage::EN,
            operations_supported: vec![Operation::PrintJob],
            pdl_override_supported: PdlOverride::Attempted,
            printer_is_accepting_jobs: true,
            printer_name: String::from("Default Printer Name"),
            printer_state: PrinterState::Idle,
            printer_state_reasons: vec![PrinterStateReason { keyword: PrinterStateReasonKeyword::None, severity: None }],
        }
    }
}
