use crate::profiles::{Profile, ProfileManager};
use crate::releases::{Release, ReleaseResponse};

mod releases;
mod utils;
mod profiles;

fn main() {
    // let mut manager = ProfileManager::new().unwrap();
    // let result = Profile::create("data-176806584");
    //
    // let profile = if result.is_err() {
    //     Profile::new("data-176806584")
    // } else {
    //     result.unwrap()
    // };
    //
    // let profile_ref = manager.add(profile).clone();
    // manager.switch_to(&profile_ref).unwrap();

    let result = ReleaseResponse::fetch_multiple();
    let mut releases = Vec::with_capacity(result.len());

    for response in &result[0..2] {
        // response.desktop_asset().unwrap().download_default(&response.tag_name).unwrap();
        releases.push(Release::from(response.to_owned()));
    }

    for release in releases {
        release.launch().expect("neeeeeeerd");
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{remove_file, File};
    use std::os::unix::fs::MetadataExt;
    use crate::releases::ReleaseResponse;
    #[test]
    fn get_releases() {
        let releases = ReleaseResponse::fetch_multiple();
        assert_ne!(releases[0].url, "");
    }

    #[test]
    fn download_release() {
        const PATH: &str = "./Mindustry.jar";

        println!("Downloading jar...");

        let releases = ReleaseResponse::fetch_multiple();
        let mut file_asset = None;

        for asset in &releases[0].assets {
            if asset.name.contains("Mindustry") {
                file_asset = Some(asset);
                break;
            }
        }

        assert!(file_asset.is_some());

        let file_result = File::create(PATH);
        assert!(file_result.is_ok());

        let file = file_result.unwrap();
        let result = file_asset.unwrap().download(&file);
        assert!(result.is_ok());
        assert_ne!(file.metadata().unwrap().size(), 0);

        remove_file(PATH).expect("Downloaded successfully, but could not delete file.");
        println!("Download complete.")
    }
}
