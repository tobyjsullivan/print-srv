#[derive(Clone, Copy, Debug)]
pub enum MimeMediaType {
    Html,
    PlainText,
    PlainTextAscii,
    PlainTextLatin1,
    PlainTextUtf8,
    Postscript,
    HpPcl,
    Pdf,
    AutoSense,
}

impl From<MimeMediaType> for String {
    fn from(m: MimeMediaType) -> Self {
        match m {
            MimeMediaType::Html => String::from("text/html"),
            MimeMediaType::PlainText => String::from("text/plain"),
            MimeMediaType::PlainTextAscii => String::from("text/plain; charset = US-ASCII"),
            MimeMediaType::PlainTextLatin1 => String::from("text/plain; charset = ISO-8859-1"),
            MimeMediaType::PlainTextUtf8 => String::from("text/plain; charset = utf-8"),
            MimeMediaType::Postscript => String::from("application/postscript"),
            MimeMediaType::HpPcl => String::from("application/vnd.hp-PCL"),
            MimeMediaType::Pdf => String::from("application/pdf"),
            MimeMediaType::AutoSense => String::from("application/octet-stream"),
        }
    }
}