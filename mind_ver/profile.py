import configparser
import time
import shutil
from dataclasses import dataclass
from pathlib import Path

# TODO: make this proper
MINDUSTRY_DIR = Path(f"{Path.home()}/.local/share/Mindustry/").resolve()
if not MINDUSTRY_DIR.exists():
    raise FileNotFoundError("Couldn't find Mindustry save folder.")

PROFILE_DATA = Path(f"{MINDUSTRY_DIR}/PROFILE_DATA.ini")
if not PROFILE_DATA.exists():
    PROFILE_DATA.write_text(f"[profile]\nname=data-{int(time.time())}")

PROFILE_CONFIG = configparser.ConfigParser()
PROFILE_CONFIG.read(PROFILE_DATA)

PROFILES_DIR = Path(f"{MINDUSTRY_DIR}/../mind_ver/profiles/")
if not PROFILES_DIR.exists():
    Path(f"{MINDUSTRY_DIR}/../mind_ver").mkdir(exist_ok=True)
    PROFILES_DIR.mkdir()


@dataclass
class Profile:
    name: str

    @staticmethod
    def new_profile(name: str) -> "Profile":
        path = Path(f"{PROFILES_DIR}/{name}")
        if not path.exists():
            path.mkdir()
            Path(f"{path}/PROFILE_DATA.ini").write_text(f"[profile]\nname={name}")

        return Profile(name)


class ProfileManager:
    __profiles: dict[str, Profile]
    __current_profile: str

    def __init__(self):
        self.__profiles = dict()
        self.__current_profile = self.__find_current_profile()
        self.add_profile(Profile(self.__current_profile))

    def __find_current_profile(self) -> str:
        return PROFILE_CONFIG.get("profile", "name")

    def add_profile(self, profile: Profile):
        self.__profiles[profile.name] = profile

    def get_profile(self, name: str) -> Profile:
        return self.__profiles[name]

    def switch_profile(self, new_profile: Profile):
        if new_profile.name not in self.__profiles:
            raise Exception("Cannot switch profile. Profile doesn't exist in manager.")
        elif new_profile.name == self.__current_profile:
            return

        current_profile = self.__profiles[self.__current_profile].name
        path = Path(f"{PROFILES_DIR}/{new_profile.name}")
        if not path.exists():
            raise FileNotFoundError("Couldn't find the directory for the target profile.")

        shutil.copytree(MINDUSTRY_DIR, f"{PROFILES_DIR}/{current_profile}")
        shutil.rmtree(MINDUSTRY_DIR)
        shutil.copytree(path.resolve(), MINDUSTRY_DIR)
        shutil.rmtree(path.resolve())

        self.__current_profile = new_profile.name

    def get_all_profiles(self):
        return list(self.__profiles.values())


# def switch_to(profile: Profile):
#     if MINDUSTRY_DIR.absolute() == profile.current_path.absolute():
#         return
#
#     shutil.copy(MINDUSTRY_DIR, PROFILES_DIR)
#
#
#
# def duplicate_profile(profile: Profile):
#     pass
