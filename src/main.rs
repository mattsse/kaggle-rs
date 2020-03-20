use kaggle::request::DatasetsList;
use kaggle::KaggleApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kaggle: KaggleApiClient = KaggleApiClient::builder().build()?;
    let resp = kaggle
        .datasets_list(&DatasetsList::builder().search("demographics"))
        .await?;

    dbg!(resp);

    Ok(())
}
