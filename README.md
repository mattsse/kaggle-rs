# kaggle-rs - accessing kaggle.com the rust way

[![Build Status](https://travis-ci.com/mattsse/kaggle-rs.svg?branch=master)](https://travis-ci.com/mattsse/kaggle-rs)
[![Crates.io](https://img.shields.io/crates/v/kaggle.svg)](https://crates.io/crates/kaggle)
[![Documentation](https://docs.rs/kaggle/badge.svg)](https://docs.rs/kaggle)

Unofficial rust implementation of the [kaggle-api](https://github.com/Kaggle/kaggle-api).

## Example

Download the newest version of a complete dataset

```rust
use kaggle::KaggleApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kaggle: KaggleApiClient = KaggleApiClient::builder().build()?;
    let dataset = kaggle
        .dataset_download_all_files("unanimad/dataisbeautiful", None, None)
        .await?;
    kaggle::archive::unzip(dataset, ".")?;
    Ok(())
}
```

## Documentation

Full docs available at [docs.rs](https://docs.rs/kaggle)

## License

The Kaggle API is released under the [Apache License, Version 2.0](LICENSE)