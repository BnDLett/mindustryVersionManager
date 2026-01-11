import subprocess
import sys
from pathlib import Path
from threading import Thread

from PySide6 import QtWidgets, QtCore
from PySide6.QtGui import QIcon, QFontMetrics

from mind_ver.profile import ProfileManager, PROFILES_DIR, Profile
from mind_ver.retrieve import retrieve_releases, Release, download_release, RELEASES_DIR


class MindVer(QtWidgets.QWidget):
    def __init__(self, profiles: ProfileManager):
        super().__init__()
        self.profiles = profiles

        self.profile_dropdown = QtWidgets.QComboBox()
        self.refresh_profiles()

        self.version_dropdown = QtWidgets.QComboBox()
        self.refresh_releases()

        self.download_form = QtWidgets.QHBoxLayout()
        self.construct_download_form()

        self.profile_form = QtWidgets.QHBoxLayout()
        self.construct_profile_form()

        self.launch_button = QtWidgets.QPushButton()
        self.launch_button.setText("Launch")
        self.launch_button.clicked.connect(self.launch_callback)


        self.layout = QtWidgets.QVBoxLayout(self)

        self.layout.addLayout(self.download_form)
        self.layout.addLayout(self.profile_form)

        self.layout.addWidget(self.profile_dropdown)
        self.layout.addWidget(self.version_dropdown)
        self.layout.addWidget(self.launch_button)

    def refresh_releases(self):
        self.version_dropdown.clear()

        for version in discover_versions():
            self.version_dropdown.addItem(version.name, version)

    def refresh_profiles(self):
        self.profile_dropdown.clear()
        for profile in self.profiles.get_all_profiles():
            self.profile_dropdown.addItem(profile.name, profile)

    def construct_download_form(self):
        self.download_input = QtWidgets.QComboBox()
        for release in retrieve_releases():
            self.download_input.addItem(release.name.removesuffix(".jar"), release)

        self.download_confirmation = QtWidgets.QPushButton("Download")
        self.download_confirmation.clicked.connect(self.download_callback)
        self.download_form.addWidget(self.download_input, 3)
        self.download_form.addWidget(self.download_confirmation, 1)

    def construct_profile_form(self):
        self.profile_name = QtWidgets.QPlainTextEdit(placeholderText="New profile")
        # self.profile_name.setFixedHeight(int(QFontMetrics(app.font()).height() * 1.6))
        self.profile_name.setFixedHeight(28)

        self.profile_confirm = QtWidgets.QPushButton("Create profile")
        self.profile_confirm.clicked.connect(self.profile_callback)

        self.profile_form.addWidget(self.profile_name, 3)
        self.profile_form.addWidget(self.profile_confirm, 1)

    @QtCore.Slot()
    def launch_callback(self):
        self.hide()

        selected_profile: Profile = self.profile_dropdown.currentData()
        selected_version: Path = self.version_dropdown.currentData()
        self.profiles.switch_profile(selected_profile)
        print(f"Running profile {selected_profile.name}")

        subprocess.run(["java", "-jar", selected_version.absolute()])
        self.show()

    @QtCore.Slot()
    def download_callback(self):
        progress = QtWidgets.QProgressBar()
        selected_version: Release = self.download_input.currentData()

        self.layout.addWidget(progress)
        download_thread = Thread(target=download_release, args=[selected_version, progress, self.refresh_releases])
        download_thread.start()

    @QtCore.Slot()
    def profile_callback(self):
        profile_name = self.profile_name.toPlainText()
        new_profile = Profile.new_profile(profile_name)
        self.profiles.add_profile(new_profile)
        self.profile_name.clear()
        self.refresh_profiles()


def discover_profiles() -> list[Profile]:
    profiles = []
    for profile in PROFILES_DIR.iterdir():
        if not profile.is_dir(): continue
        profiles.append(Profile(profile.name))

    return profiles


# def get_release(release: Release):
#     releases_folder = Path("releases")
#     releases_folder.mkdir(exist_ok=True)
#
#     release_file = Path(f"{releases_folder}/{release.tag}.jar")
#     if not release_file.exists(): download_release(release)

def discover_versions() -> list[Path]:
    if not RELEASES_DIR.exists(): return []

    releases = []
    for release in RELEASES_DIR.iterdir():
        if release.is_dir(): continue
        print(f"Discovered release: {release.name}")
        releases.append(release)

    return releases


def main():
    manager = ProfileManager()

    for discovered_profile in discover_profiles():
        print(f"Discovered profile: {discovered_profile.name}")
        manager.add_profile(discovered_profile)

    app = QtWidgets.QApplication([])

    widget = MindVer(manager)
    widget.resize(400, 0)  # I don't care about the actual Y, but I do care about the X
    widget.setWindowTitle("Mindustry Version Manager")
    widget.setWindowIcon(QIcon(f"{Path.home()}/.local/share/icons/hicolor/48x48/apps/mind_ver.png"))
    widget.show()

    sys.exit(app.exec())


if __name__ == "__main__":
    main()
