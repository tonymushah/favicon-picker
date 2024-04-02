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

/// This function gives you the default favicon [`Url`] from a given url 
/// 
/// Example:
/// 
/// ```
/// 
/// use favicon_picker::get_default_favicon_url;
/// 
/// use url::Url;
/// 
/// let base_url = Url::parse("https://crates.io").unwrap();
/// 
/// assert_eq!(Url::parse("https://crates.io/favicon.ico").unwrap(), get_default_favicon_url(&base_url).unwrap())
/// 
/// ```
/// 
pub fn get_default_favicon_url(base_url: &Url) -> std::result::Result<Url, url::ParseError> {
    base_url.join("/favicon.ico")
}

/// This function gives you the default favicon [`Favicon`] from a given url 
pub fn get_default_favicon(base_url: &Url) -> Result<Favicon> {
    Ok(Favicon {
        href: get_default_favicon_url(base_url)?,
        size: Default::default(),
        type_: Default::default()
    })
}

pub async fn get_favicons_from_url(
    client: &reqwest::Client,
    base_url: &Url,
) -> Result<Vec<Favicon>> {
    let res = client.get(base_url.clone()).send().await?;
    if !res.status().is_success() {
        return Err(Error::ReqwestSend { code: res.status() });
    }
    let html_raw = res.text().await?;
    let html = Html::parse_document(&html_raw);
    let icons = InnerFavicon::extract_favicons(&html)?
        .into_iter()
        .flat_map(|e| <Favicon as TryFrom<(&Url, InnerFavicon<'_>)>>::try_from((base_url, e)))
        .collect::<Vec<Favicon>>();
    if icons.is_empty() {
        get_default_favicon(base_url).map(|r| vec![r])
    } else {
        Ok(icons)
    }
}

#[cfg(feature = "blocking")]
pub fn get_blocking_favicons_from_url(
    client: &reqwest::blocking::Client,
    base_url: &Url,
) -> Result<Vec<Favicon>> {
    let res = client.get(base_url.clone()).send()?;
    if !res.status().is_success() {
        return Err(Error::ReqwestSend { code: res.status() });
    }
    let html_raw = res.text()?;
    let html = Html::parse_document(&html_raw);
    let icons = InnerFavicon::extract_favicons(&html)?
        .into_iter()
        .flat_map(|e| <Favicon as TryFrom<(&Url, InnerFavicon<'_>)>>::try_from((base_url, e)))
        .collect::<Vec<Favicon>>();
    if icons.is_empty() {
        get_default_favicon(base_url).map(|r| vec![r])
    } else {
        Ok(icons)
    }
}
