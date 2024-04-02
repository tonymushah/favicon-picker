use reqwest::StatusCode;
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
    Reqwest(#[from] reqwest::Error),
    #[error("No elements was been found with the selector `link[rel='icon']`")]
    NoLinkIconElements,
    #[error("We got a {code} while fetching for the icons bytes")]
    ReqwestSend {
        code: StatusCode
    }
}
