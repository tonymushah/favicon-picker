# favicon-picker

This is just a small library that allows you to get websites [favicons](https://en.wikipedia.org/wiki/Favicon).

This library is similar to [`site_icons`](https://github.com/samdenty/site_icons) but i just wanted something simple and flexible to use for various side-project.

## Example

### Simple

```rust
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
        // I just printed the value to the standard output
        println!("{:#?}", favs);
    }
    Ok(())
}

```

### Download

```rust
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
    /// get existing favicons on the page
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
```

## How does it work ??

When you call the [`favicon_picker::get_favicons_from_url`](https://github.com/tonymushah/favicon-picker/blob/main/src/lib.rs#L48), it will just fetch, and parse the existing HTML page. Then it will lookup to existing `<link rel='icon'/>` and that's it.

If it finds no favicon, it will give you the default favicon url which is `http://<some-website-domain>/favicon.ico`

## Feature flags

- `non_exhaustive`: Will make the `Favicon` struct `#[non_exhaustive]` *Enabled by default*
- `serde`: Allows the `Favicon` struct to be serialized and deserialized with the [`serde`](https:/serde.rs) framework
- `blocking`: Allows you to use the `reqwest::blocking::Client`. Useful if you don't want to use `async/await`

## Contribution

All pull request are welcome!!

## License

GNU GENERAL PUBLIC LICENSE version 3
