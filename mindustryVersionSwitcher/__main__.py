from mindustryVersionSwitcher.retriever import retrieve_releases

if __name__ == "__main__":
    releases = retrieve_releases()

    for release in releases:
        print(f"{release.name.strip()}:\n\tTag: {release.tag}\n\tURL: {release.url}")
