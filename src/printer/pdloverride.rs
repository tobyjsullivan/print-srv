#[derive(Copy, Clone, Debug)]
pub enum PdlOverride {
    Attempted,
    NotAttempted,
}

impl From<PdlOverride> for String {
    fn from(po: PdlOverride) -> Self {
        match po {
            PdlOverride::Attempted => String::from("attempted"),
            PdlOverride::NotAttempted => String::from("not-attempted"),
        }
    }
}
