use std::{
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::Path,
};

use favicon_picker::get_favicons_from_url;
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};
use url::Url;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let dowload_path = Path::new("output");
    create_dir_all(dowload_path)?;
    let client = Client::builder()
        .default_headers({
            // the `User-Agent` is not required here
            let mut headers = HeaderMap::new();
            headers.append(USER_AGENT, HeaderValue::from_str("favicon-picker/1.0.0")?);
            headers
        })
        .build()?;
    let base_url = Url::parse("https://comic-walker.com/")?;
    for favs in get_favicons_from_url(&client, &base_url)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?
    {
        // i just need the favicon file name here
        let file_name = favs
            .href
            .path_segments()
            .ok_or_else(|| anyhow::Error::msg("cannot be base"))?
            .last()
            .ok_or_else(|| anyhow::Error::msg("can't get the last url fragments"))?;
        let filepath = dowload_path.join(file_name);
        let data = favs.get_image_bytes(&client).await?;
        let mut file = BufWriter::new(File::create(filepath)?);
        file.write_all(&data)?;
        file.flush()?;
    }
    Ok(())
}
