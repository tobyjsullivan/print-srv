#[derive(Clone, Debug)]
pub struct PrinterUri {
    pub uri: String,
    pub authentication: UriAuthenticationMethod,
    pub security: UriSecurityMethod,
}

impl PrinterUri {
    pub fn new(
        uri: &str,
        authentication: UriAuthenticationMethod,
        security: UriSecurityMethod,
    ) -> Self {
        Self {
            uri: String::from(uri),
            authentication,
            security,
        }
    }
}

// https://tools.ietf.org/html/rfc8011#section-5.4.2
#[derive(Copy, Clone, Debug)]
pub enum UriAuthenticationMethod {
    None,
    RequestingUserName,
    Basic,
    Digest,
    Certificate,
}

impl From<UriAuthenticationMethod> for String {
    fn from(m: UriAuthenticationMethod) -> Self {
        match m {
            UriAuthenticationMethod::None => String::from("none"),
            UriAuthenticationMethod::RequestingUserName => String::from("requesting-user-name"),
            UriAuthenticationMethod::Basic => String::from("basic"),
            UriAuthenticationMethod::Digest => String::from("digest"),
            UriAuthenticationMethod::Certificate => String::from("certificate"),
        }
    }
}

// https://tools.ietf.org/html/rfc8011#section-5.4.3
#[derive(Copy, Clone, Debug)]
pub enum UriSecurityMethod {
    None,
    Tls,
}

impl From<UriSecurityMethod> for String {
    fn from(s: UriSecurityMethod) -> Self {
        match s {
            UriSecurityMethod::None => String::from("none"),
            UriSecurityMethod::Tls => String::from("tls"),
        }
    }
}
