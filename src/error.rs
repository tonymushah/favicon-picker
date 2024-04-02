use scraper::error;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    #[error("{0}")]
    SelectorKind(#[from] error::SelectorErrorKind<'static>)
}
