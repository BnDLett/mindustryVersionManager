import os
import shutil
import subprocess
import sys
from pathlib import Path


def install_app():
    directory = Path(os.path.realpath(__file__)).parent
    shutil.copytree(directory, f"{Path.home()}/.local/bin/mind_ver", dirs_exist_ok=True)
    shutil.copy(f"{directory.parent}/mind_ver.desktop", f"{Path.home()}/.local/share/applications/mind_ver.desktop")
    shutil.copy(f"{directory}/assets/alphaaaa.png", f"{Path.home()}/.local/share/icons/hicolor/48x48/apps/mind_ver.png")

    subprocess.run(["/usr/bin/python3", "-m", "pip", "install", "--break-system-packages", "-r",
                    f"{directory.parent}/requirements.txt"])


def uninstall_app():
    shutil.rmtree(f"{Path.home()}/.local/bin/mind_ver")
    os.remove(f"{Path.home()}/.local/share/applications/mind_ver.desktop")
    # os.remove("/usr/share/pixmaps/mind_ver.png")


def reinstall_app():
    uninstall_app()
    install_app()


if __name__ == "__main__":
    if len(sys.argv) != 1:
        if sys.argv[1] == "install": install_app()
        elif sys.argv[1] == "uninstall": uninstall_app()
        elif sys.argv[1] == "reinstall": reinstall_app()
        sys.exit(0)

    from mind_ver import gui
    gui.main()
