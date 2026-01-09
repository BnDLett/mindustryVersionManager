from urllib import parse
import requests
import json

RELEASES_URL = "https://api.github.com/repos/anuken/mindustry/releases"


class Release:
    name: str
    tag: str
    url: str

    def __init__(self, name: str, tag: str, url: str):
        self.name = name
        self.tag = tag

        parsed_url = parse.urlparse(url)
        if parsed_url.scheme != "https":
            raise ValueError(f"HTTPS required, received {parsed_url.scheme}")

        self.url = url


def retrieve_releases() -> list[Release]:
    """
    Retrieves a list of **all** releases.
    """
    data = requests.get(RELEASES_URL)
    releases = []

    if data.status_code != 200:
        raise ConnectionError("Received non-200 status code.")

    json_data = json.loads(data.content)
    for release_json in json_data:
        release = Release(release_json["name"], release_json["tag_name"], release_json["url"])
        releases.append(release)

    return releases


def download_release(release: Release):
    pass
