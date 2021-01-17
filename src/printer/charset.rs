use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub enum Charset {
    Utf8,
}

impl From<Charset> for String {
    fn from(c: Charset) -> Self {
        match c {
            Charset::Utf8 => String::from("utf-8"),
        }
    }
}

impl TryFrom<String> for Charset {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.to_lowercase() == "utf-8" {
            Ok(Charset::Utf8)
        } else {
            Err(format!("Unknown Charset {}", value))
        }
    }
}