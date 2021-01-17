#[derive(Copy, Clone, Debug)]
pub enum NaturalLanguage {
    EN,
}

impl From<NaturalLanguage> for String {
    fn from(lang: NaturalLanguage) -> Self {
        match lang {
            NaturalLanguage::EN => String::from("en"),
        }
    }
}