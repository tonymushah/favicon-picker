use scraper::error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SelectorKind(#[from] error::SelectorErrorKind<'static>)
}
