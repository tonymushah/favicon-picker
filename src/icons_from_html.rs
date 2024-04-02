use scraper::{ElementRef, Html};

use crate::{selectors::favicon_selector, Result};

///
/// This function allows you to get all `<link rel='icon'>` on an [`Html`] page.
/// 
/// Example:
/// 
/// ```rust
///
/// use favicon_picker::extract_icon_links_from_html;
/// use scraper::Html;
/// 
/// let html_raw = "
///     <!DOCTYPE html>
///     <html>
///         <head>
///             <link rel='icon' size='96x96' href='/some-favicon.ico'/>
///             <link rel='icon' size='any' href='/favicon.ico'/>
///         </head>
///     </html>
/// ";
/// let html = Html::parse_document(html_raw);
/// let icons = extract_icon_links_from_html(&html).unwrap();
/// assert_eq!(icons.len(), 2);
/// 
/// ```
/// 
pub fn extract_icon_links_from_html(html: &Html) -> Result<Vec<ElementRef<'_>>> {
    let selector = favicon_selector()?;
    Ok(html.select(&selector).collect())
}

#[cfg(test)]
mod tests {
    use scraper::Html;

    use crate::extract_icon_links_from_html;

    #[test]
    fn extract_from_html() -> anyhow::Result<()> {
        let html_raw = "
            <!DOCTYPE html>
            <html>
                <head>
                    <link rel='icon' size='96x96' href='/some-favicon.ico'/>
                    <link rel='icon' size='any' href='/favicon.ico'/>
                </head>
            </html>
        ";
        let html = Html::parse_document(html_raw);
        let icons = extract_icon_links_from_html(&html).map_err(|e| anyhow::Error::msg(e.to_string()))?;
        assert_eq!(icons.len(), 2);
        Ok(())
    }
}