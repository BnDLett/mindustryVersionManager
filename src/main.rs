mod fetcher;

fn main() {

}

#[cfg(test)]
mod tests {
    use std::fs::{remove_file, File};
    use std::os::unix::fs::MetadataExt;
    use crate::fetcher::Release;
    #[test]
    fn get_releases() {
        let releases = Release::fetch_multiple();
        assert_ne!(releases[0].url, "");
    }

    #[test]
    fn download_release() {
        const PATH: &str = "./Mindustry.jar";

        println!("Downloading jar...");

        let releases = Release::fetch_multiple();
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
