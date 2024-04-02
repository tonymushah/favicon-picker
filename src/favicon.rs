use bytes::Bytes;
use reqwest::{Client, Url};
use scraper::{ElementRef, Html};

use crate::{extract_icon_links_from_html, Result};

#[derive(Debug, Clone, Copy)]
pub struct InnerFavicon<'a> {
    pub href: &'a str,
    pub size: Option<&'a str>,
    pub type_: Option<&'a str>,
}

impl<'a> TryFrom<ElementRef<'a>> for InnerFavicon<'a> {
    type Error = crate::error::Error;
    fn try_from(value: ElementRef<'a>) -> std::prelude::v1::Result<Self, Self::Error> {
        let href = value
            .attr("href")
            .ok_or(crate::error::Error::HrefNotFound)?;
        Ok(Self {
            href,
            size: value.attr("size"),
            type_: value.attr("type"),
        })
    }
}

impl<'a> InnerFavicon<'a> {
    pub fn get_href_url(&'a self, base_url: &Url) -> Result<Url> {
        Ok(base_url.join(self.href)?)
    }
    pub fn extract_favicons(html: &'a Html) -> Result<Vec<InnerFavicon<'a>>> {
        let elements = extract_icon_links_from_html(html)?;
        Ok(elements
            .into_iter()
            .flat_map(<Self as TryFrom<ElementRef<'a>>>::try_from)
            .collect())
    }
}

/// This is the instance of an website favicon 
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde:Serialize, serde::Deserialize))]
pub struct Favicon {
    /// the icon url
    pub href: Url,
    /// the icon size
    pub size: Option<String>,
    /// the icon type
    pub type_: Option<String>,
}

impl TryFrom<(&Url, InnerFavicon<'_>)> for Favicon {
    type Error = crate::error::Error;
    fn try_from(
        (base_url, fav): (&Url, InnerFavicon<'_>),
    ) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Self {
            href: fav.get_href_url(base_url)?,
            size: fav.size.map(String::from),
            type_: fav.size.map(String::from),
        })
    }
}

impl TryFrom<(&Url, ElementRef<'_>)> for Favicon {
    type Error = crate::error::Error;
    fn try_from(
        (base_url, fav): (&Url, ElementRef<'_>),
    ) -> std::prelude::v1::Result<Self, Self::Error> {
        let inner: InnerFavicon<'_> = fav.try_into()?;
        (base_url, inner).try_into()
    }
}

impl Favicon {
    /// Return the favicon [`Bytes`]
    /// This method uses the default [`reqwest::Client`]. 
    /// If you want to use the [`reqwest::blocking::Client`], use the [`Favicon::get_blocking_images_bytes`] instead
    pub async fn get_image_bytes(&self, client: &Client) -> reqwest::Result<Bytes> {
        client.get(self.href.clone()).send().await?.bytes().await
    }
    /// Same as [`Favicon::get_image_bytes`] but it uses the [`reqwest::blocking::Client`]
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg("blocking")))]
    pub fn get_blocking_images_bytes(&self, client: &reqwest::blocking::Client) -> reqwest::Result<Bytes> {
        client.get(self.href.clone()).send()?.bytes()
    }
}