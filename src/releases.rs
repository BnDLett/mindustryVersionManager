use std::fs::File;
use std::io::{Write};
use std::path::Path;
use std::process::{Command, Output};
use const_format::concatcp;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use resolve_path::PathResolveExt;
use serde::Deserialize;
use crate::utils::CONFIG_PATH;

const RELEASES_URL: &str = "https://api.github.com/repos/anuken/mindustry/releases";
const AGENT_NAME: &str = "Mindustry Version Manager";
const RELEASES_PATH: &str = concatcp!(CONFIG_PATH, "/releases");

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

    pub fn desktop_asset(&self) -> Option<&Asset> {
        for asset in &self.assets {
            if asset.name != "Mindustry.jar" { continue; }

            return Some(asset);
        }

        None
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

    /// Downloads the asset to a default path.
    pub(crate) fn download_default(&self, name: &String) -> Result<(), Option<reqwest::Error>> {
        let path_str = &*format!("{}/{}.jar", RELEASES_PATH, &*name);
        let path = Path::new(path_str);
        let resolved = path.resolve();

        let file_result = File::create(resolved);
        if file_result.is_err() { return Err(None) }
        let file = file_result.unwrap();

        self.download(&file)
    }
}

pub struct Release {
    path: String,
    name: String,
}

impl Release {
    pub fn new<'a>(path: &String, name: &String) -> Release {
        Release {
            path: path.clone(),
            name: name.clone()
        }
    }

    pub fn launch(&self) -> std::io::Result<Output> {
        let path = Path::new(&*self.path);
        // Command::new("chmod")
        //     .arg("+x")
        //     .arg(path)
        //     .output()?;
        Command::new("java")
            .arg("-jar")
            .arg(path)
            .output()
    }
}

// lifetimes are a mf
impl From<&ReleaseResponse> for Release {
    fn from(resp: &ReleaseResponse) -> Release {
        let path_str = &*format!("{}/{}.jar", RELEASES_PATH, &*resp.tag_name);
        let path = Path::new(path_str);
        let resolved = path.resolve();

        Release {
            name: resp.name.clone(),
            path: String::from(resolved.to_str().unwrap())
        }
    }
}
