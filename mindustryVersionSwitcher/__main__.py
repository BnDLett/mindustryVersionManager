from mindustryVersionSwitcher.retriever import retrieve_releases, download_release

if __name__ == "__main__":
    releases = retrieve_releases()

    for release in releases[:1]:
        # print(f"{release.name.strip()}:\n\tTag: {release.tag}\n\tURL: {release.download_url}")
        download_release(release)
