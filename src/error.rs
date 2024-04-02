use scraper::error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    SelectorKind(#[from] error::SelectorErrorKind<'static>),
    #[error("{0}")]
    UrlParse(#[from] url::ParseError),
    #[error("the `href` attribute is not found in element_ref")]
    HrefNotFound,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error)
}
