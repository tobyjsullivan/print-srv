mod charset;
mod compression;
mod mimemediatype;

use std::convert::TryFrom;
pub use charset::Charset;
pub use compression::Compression;
pub use mimemediatype::MimeMediaType;

#[derive(Clone, Debug)]
pub struct Printer {
    pub charset_configured: Charset,
    pub charset_supported: Vec<Charset>,
    pub compression_supported: Vec<Compression>,
    pub document_format_default: MimeMediaType,
    pub document_format_supported: Vec<MimeMediaType>,
    // 	EXPECT document-format-supported
    // 	EXPECT generated-natural-language-supported
    // 	EXPECT ipp-versions-supported
    // 	EXPECT natural-language-configured
    // 	EXPECT operations-supported
    // 	EXPECT pdl-override-supported
    // 	EXPECT printer-is-accepting-jobs
    // 	EXPECT printer-name
    // 	EXPECT printer-state
    // 	EXPECT printer-state-reasons
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
        }
    }
}
