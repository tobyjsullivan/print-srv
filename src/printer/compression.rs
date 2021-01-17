#[derive(Copy, Clone, Debug)]
pub enum Compression {
    None,
    Deflate,
    GZip,
    Compress,
}

impl From<Compression> for String {
    fn from(c: Compression) -> Self {
        match c {
            Compression::None => String::from("none"),
            Compression::Deflate => String::from("deflate"),
            Compression::GZip => String::from("gzip"),
            Compression::Compress => String::from("compress"),
        }
    }
}
