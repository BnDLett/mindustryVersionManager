use reqwest::Client;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

const RELEASES_URL: &str = "https://api.github.com/repos/anuken/mindustry/releases";

#[derive(Deserialize)]
pub struct Release {
    pub url: String,
    pub tag_name: String,
    pub name: String,
    pub prerelease: bool,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub body: String,
}

#[derive(Deserialize)]
pub struct Asset {
    pub url: String,
    pub name: String,
    pub size: u64,
    pub digest: String,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

impl Release {
    // TODO: do not panic.
    pub fn fetch() -> Release {
        let client = Client::new();
        let response = client
            .get(RELEASES_URL)
            .header(USER_AGENT, "Mindustry Version Manager")
            .send();

        let response_str = match response {
            Ok(resp) => resp.text().unwrap(),
            Err(err) => panic!("Error: {}", err)
        };

        serde_json::from_str(response_str).expect("Unknown error.")
    }
}
