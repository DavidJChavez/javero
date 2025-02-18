use argh::FromArgs;
use serde::Deserialize;

#[derive(FromArgs, PartialEq, Debug)]
/// List available jdk versions
#[argh(subcommand, name = "list")]
pub struct ListCommand {
}

const AZUL_ZULU_API: &str = "https://api.azul.com/metadata/v1/zulu/packages?page=1&page_size=10&arch=x86&os=windows&archive_type=zip&release_status=ga&java_package_type=jdk&javafx_bundled=false&certifications=tck&availability_types=ca";

impl ListCommand {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(AZUL_ZULU_API)
            .await?
            .text()
            .await?;
        let items: Vec<ZuluItem> = serde_json::from_str(&resp)?;
        println!("Azul Zulu:");
        for item in items.iter() {
            println!("\t{} ", item.name.split_at(item.name.rfind("-").unwrap()).0);
        }
        
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct ZuluItem {
    // arch: String,
    // archive_type: String,
    java_version: Vec<u32>,
    name: String,
    latest: bool,
    openjdk_build_number: u32,
    download_url: String,
    // os: String
}
