use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum IppVersion {
    V1_0,
    V1_1,
    V2_0,
    V2_1,
    V2_2,
}

impl From<IppVersion> for String {
    fn from(v: IppVersion) -> Self {
        match v {
            IppVersion::V1_0 => String::from("1.0"),
            IppVersion::V1_1 => String::from("1.1"),
            IppVersion::V2_0 => String::from("2.0"),
            IppVersion::V2_1 => String::from("2.1"),
            IppVersion::V2_2 => String::from("2.2"),
        }
    }
}

impl Display for IppVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str: String = (*self).into();
        write!(f, "{}", str)
    }
}