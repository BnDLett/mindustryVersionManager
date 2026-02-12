from pathlib import Path
from typing import Callable
from urllib import parse
import requests
import json

from PySide6.QtWidgets import QProgressBar

RELEASES_URL = "https://api.github.com/repos/anuken/mindustry/releases"
MINDUSTRY_DIR = Path(f"{Path.home()}/.local/share/Mindustry/")
RELEASES_DIR = Path(f"{MINDUSTRY_DIR}/../mind_ver/releases/")
RELEASES_DIR.mkdir(exist_ok=True)


class MissingJarException(Exception):
    pass


class Release:
    name: str
    tag: str
    download_url: str
    size: int

    def __init__(self, name: str, tag: str, download_url: str, size: int):
        self.name = name
        self.tag = tag

        parsed_url = parse.urlparse(download_url)
        if parsed_url.scheme != "https":
            raise ValueError(f"HTTPS required, received {parsed_url.scheme}")

        self.download_url = download_url
        self.size = size


# this is just a helper function
def __retrieve_download_data(data: dict) -> tuple[str, int]:
    for asset in data["assets"]:
        if asset["name"] == "Mindustry.jar":
            return asset["browser_download_url"], asset["size"]

    raise MissingJarException("Couldn't find jar asset.")


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
        try:
            release = Release(release_json["name"], release_json["tag_name"], *__retrieve_download_data(release_json))
            releases.append(release)
        except MissingJarException:
            print(f"Couldn't retrieve Jar asset for {release_json["tag_name"]}.")

    return releases


def download_release(release: Release, progress: QProgressBar = None, refresh_callback: Callable[[], None] = None):
    jar_data = requests.get(release.download_url, stream=True)

    if not jar_data.ok:
        raise ConnectionError("Received non-ok status code.")

    with open(f"{RELEASES_DIR}/{release.tag}.jar", "wb") as jar_file:
        chunk_counter = 0
        CHUNK_SIZE = 1_000_000

        for chunk in jar_data.iter_content(chunk_size=CHUNK_SIZE):
            jar_file.write(chunk)

            chunk_counter += 1
            percentage = ((chunk_counter * CHUNK_SIZE) / release.size) * 100

            print(f"\rDownloading {release.tag}... {percentage:.0f}% complete.".ljust(200, " "),
                  flush=True, end="")
            if progress is not None: progress.setValue(int(percentage))

        if progress is not None:
            progress.destroy()
            progress.hide()
            del progress
            if refresh_callback is not None: refresh_callback()

        print()
