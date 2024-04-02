pub mod error;
pub(crate) mod favicon;
mod icons_from_html;
pub mod selectors;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

use error::Error;
use favicon::InnerFavicon;
pub use icons_from_html::*;

pub use favicon::Favicon;
use scraper::Html;
use url::Url;

pub fn get_default_favicon_url(base_url: &Url) -> std::result::Result<Url, url::ParseError> {
    base_url.join("/favicon.ico")
}

pub async fn get_favicons_from_url(
    client: &reqwest::Client,
    base_url: &Url,
) -> Result<Vec<Favicon>> {
    let html_raw = client.get(base_url.clone()).send().await?.text().await?;
    let html = Html::parse_document(&html_raw);
    let icons = InnerFavicon::extract_favicons(&html)?
        .into_iter()
        .flat_map(|e| <Favicon as TryFrom<(&Url, InnerFavicon<'_>)>>::try_from((base_url, e)))
        .collect::<Vec<Favicon>>();
    if icons.is_empty() {
        Err(Error::NoLinkIconElements)
    } else {
        Ok(icons)
    }
}

#[cfg(feature = "blocking")]
pub fn get_blocking_favicons_from_url(
    client: &reqwest::blocking::Client,
    base_url: &Url,
) -> Result<Vec<Favicon>> {
    let html_raw = client.get(base_url.clone()).send()?.text()?;
    let html = Html::parse_document(&html_raw);
    let icons = InnerFavicon::extract_favicons(&html)?
        .into_iter()
        .flat_map(|e| <Favicon as TryFrom<(&Url, InnerFavicon<'_>)>>::try_from((base_url, e)))
        .collect::<Vec<Favicon>>();
    if icons.is_empty() {
        Err(Error::NoLinkIconElements)
    } else {
        Ok(icons)
    }
}
