use std::fs::File;
use std::io::{Write};
use std::path::Path;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

const RELEASES_URL: &str = "https://api.github.com/repos/anuken/mindustry/releases";
const AGENT_NAME: &str = "Mindustry Version Manager";

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ReleaseResponse {
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

impl ReleaseResponse {
    // TODO: do not panic.
    pub fn fetch_multiple() -> Vec<ReleaseResponse> {
        let client = Client::new();
        let response = client
            .get(RELEASES_URL)
            .header(USER_AGENT, AGENT_NAME)
            .send();

        let response_str = match response {
            Ok(resp) => resp.text().unwrap(),
            Err(err) => panic!("Error: {}", err)
        };

        // println!("{}", response_str);
        serde_json::from_str(&*response_str).expect("Unknown error.")
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Asset {
    pub url: String,
    pub name: String,
    pub size: u64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

impl Asset {
    pub fn download(&self, mut location: &File) -> Result<(), Option<reqwest::Error>> {
        let client = Client::new();
        let response = client
            .get(&self.browser_download_url)
            .header(USER_AGENT, AGENT_NAME)
            .send()?;

        if response.status().is_success() {
            let data = response.bytes()?;
            location.write_all(&*data.to_vec()).unwrap();
            return Ok(());
        }

        Err(None)
    }
}

pub struct Release {
    path: Path,
    name: String,
}

impl Release {
    pub fn new(path: &Path, name: &String) -> Release {
        Release {
            path: path.clone(),
            name: name.clone()
        }
    }
}

// impl From<ReleaseResponse> for Release {
//     fn from(resp: ReleaseResponse) -> Release {
//         let path = Path::new();
//
//         Release {
//             name: resp.name
//         }
//     }
// }
