use scraper::{Selector, error::SelectorErrorKind};

pub fn favicon_selector() -> Result<Selector, SelectorErrorKind<'static>> {
    Selector::parse("link[rel='icon']")
}