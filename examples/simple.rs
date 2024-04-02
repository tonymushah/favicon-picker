use favicon_picker::get_favicons_from_url;
use reqwest::{header::{HeaderMap, HeaderValue, USER_AGENT}, Client};
use url::Url;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder().default_headers({
        // the `User-Agent` is not required here
        let mut headers = HeaderMap::new();
        headers.append(USER_AGENT, HeaderValue::from_str("favicon-picker/1.0.0")?);
        headers
    }).build()?;
    let base_url = Url::parse("https://comic-walker.com/")?;
    for favs in get_favicons_from_url(&client, &base_url).await.map_err(|e| anyhow::Error::msg(e.to_string()))? {
        println!("{:#?}", favs);
    }
    Ok(())
}