use argh::FromArgs;
use serde::Deserialize;

#[derive(FromArgs, PartialEq, Debug)]
/// List available jdk versions
#[argh(help_triggers("help"))]
#[argh(subcommand, name = "list")]
pub struct ListCommand {
    #[argh(positional)]
    /// search for a specific java version
    java_version: Option<String>,
}

const AZUL_API_URL: &str = "https://api.azul.com/metadata/v1/zulu/packages";

impl ListCommand {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let arch = match std::env::consts::ARCH {
            "x86_64" => "x64",
            arch => arch
        };
        let os = std::env::consts::OS;

        let mut req_builder = client.get(AZUL_API_URL)
        .query(&[
            ("page", "1"),
            ("page_size", "10"),
            ("arch", arch),
            ("os", os),
            ("archive_type", "zip"),
            ("release_status", "ga"),
            ("java_package_type", "jdk"),
            ("javafx_bundled", "false"),
            ("certifications", "tck"),
            ("availability_types", "ca"),
        ]);

        if let Some(java_version) = &self.java_version {
            req_builder = req_builder.query(&[("java_version", java_version)]);
        } else {
            println!("Java version not provided. Showing all options.");
        }

        let resp = req_builder.send().await?.text().await?;

        let items: Vec<ZuluItem> = serde_json::from_str(&resp)?;
        
        if items.len() > 0 {
            println!("Azul Zulu:");
            for item in items.iter() {
                println!("\t{} ", item.name.split_at(item.name.rfind("-").unwrap()).0);
            }
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct ZuluItem {
    // arch: String,
    // archive_type: String,
    // java_version: Vec<u32>,
    name: String,
    // latest: bool,
    // openjdk_build_number: u32,
    // download_url: String,
    // os: String
}
