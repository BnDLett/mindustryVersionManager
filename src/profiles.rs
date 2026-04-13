use std::collections::HashMap;
use std::fs::{create_dir, create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::{Path};
use std::time::{SystemTime, UNIX_EPOCH};
use ini::ini;
use crate::apply_attrib;
use crate::utils::{copy_tree, get_index};
use const_format::concatcp;
use resolve_path::PathResolveExt;

apply_attrib! {
    #![cfg(target_os = "linux")]

    const MINDUSTRY_PATH: &str = "~/.local/share/Mindustry";
}

const PROFILE_CONFIG: &str = concatcp!(MINDUSTRY_PATH, "/PROFILE_DATA.ini");
const CONFIG_PATH: &str = concatcp!(MINDUSTRY_PATH, "/../mind_ver");
const PROFILES_PATH: &str = concatcp!(CONFIG_PATH, "/profiles");

// #[cfg(target_os = "windows", target_os = "android")]
// compile_error!("The selected OS is not supported.");

pub struct ProfileManager {
    profiles: Vec<Profile>,
    current_profile: String,
    ini: HashMap<String, HashMap<String, Option<String>>>
}

impl ProfileManager {
    pub fn new() -> Result<ProfileManager, String> {
        let profiles_path = Path::new(PROFILES_PATH);
        create_dir_all(profiles_path.resolve()).unwrap();

        let mut manager = ProfileManager{
            profiles: Vec::new(),
            current_profile: String::new(),
            ini: HashMap::new()
        };
        manager.load_ini().unwrap();
        manager.add_current().unwrap();
        Ok(manager)
    }

    pub fn switch_to(&self, profile: &Profile) -> Result<(), &'static str> {
        let current_profile = self.current_profile.clone();

        if self.profiles.iter().find(|x| {x.name == profile.name}).is_none() {
            return Err("Profile is not loaded.");
        } else if profile.name == current_profile {
            return Ok(());
        }

        let target_path = &*format!("{}/{}", PROFILES_PATH, profile.name);
        let target_dir = Path::new(target_path);
        let target_resolved = target_dir.resolve();
        if !target_resolved.exists() || !target_resolved.is_dir() {
            return Err("The specified file path cannot be found.")
        }

        let new_path = &*format!("{}/{}", PROFILES_PATH, current_profile);
        let new_dir = Path::new(new_path);
        let new_resolved = new_dir.resolve();
        if new_resolved.exists() {
            return Err("A profile of the same name already exists (duplicate profile).");
        }

        let mindustry = Path::new(MINDUSTRY_PATH);
        let mindustry_resolved = mindustry.resolve();

        copy_tree(mindustry_resolved.clone(), new_resolved).unwrap();
        remove_dir_all(mindustry_resolved.clone()).unwrap();
        copy_tree(target_resolved.clone(), mindustry_resolved).unwrap();
        remove_dir_all(target_resolved).unwrap();

        Ok(())
    }

    pub fn add(&mut self, profile: Profile) -> &Profile {
        let name = profile.name.clone();
        self.profiles.push(profile);
        self.find(&*name).unwrap()
    }

    /// Removes a profile, but does not delete it.
    pub fn remove(&mut self, profile: Profile) -> Result<(), ()> {
        let index = get_index(&self.profiles, |x: &Profile| {
            if x.name == profile.name { return true; }
            false
        });

        self.profiles.remove(index?);
        Ok(())
    }

    pub fn find(&mut self, name: &str) -> Option<&Profile> {
        self.profiles.iter().find(|x| {
            if x.name == name {
                return true;
            }

            false
        })
    }

    fn load_ini(&mut self) -> Result<(), ()> {
        let profile_path = Path::new(PROFILE_CONFIG);
        let resolved = profile_path.resolve();
        let mut data = ini!(safe resolved.to_str().unwrap());

        if data.is_err() {
            ProfileManager::generate_ini()?;
            data = ini!(safe resolved.to_str().unwrap());
        }

        self.ini = data.unwrap();   // shouldn't be an error. If it still is, then something went
                                    // horribly wrong.
        Ok(())
    }

    fn add_current(&mut self) -> Result<(), ()> {
        let name = self.ini["profile"]["name"].clone();
        if name.is_none() { return Err(()) };

        let profile = Profile::new(&*name.unwrap());
        self.current_profile = profile.name.clone();
        self.profiles.push(profile);

        Ok(())
    }

    fn generate_ini() -> Result<(), ()> {
        let path = Path::new(PROFILE_CONFIG);
        let resolved = path.resolve();
        let mut file = File::create(resolved).unwrap();

        let time = SystemTime::now().duration_since(UNIX_EPOCH);
        let mut seconds = 0u64;
        if time.is_ok() {
            seconds = time.unwrap().as_secs();
        }

        let data = format!("[profile]\nname=data-{}", seconds);
        let result = file.write_all(data.as_bytes());

        if result.is_err() {
            return Err(());
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Profile {
    name: String
}

impl Profile {
    pub fn new(name: &str) -> Profile {
        Profile {
            name: String::from(name)
        }
    }

    pub fn create(name: &str) -> Result<Profile, &'static str> {
        let profile = Profile::new(name);
        let path_str = &*format!("{}/{}", PROFILES_PATH, profile.name);
        let path = Path::new(path_str);
        let resolved = path.resolve();

        if resolved.exists() {
            return Err("Profile already exists.");
        }

        create_dir(resolved.clone()).unwrap();

        let path_str = format!("{}/PROFILE_DATA.ini", resolved.to_str().unwrap());
        let ini_path = Path::new(&*path_str);
        let mut file = File::create(ini_path).unwrap();
        let data = format!("[profile]\nname={}", name);
        let result = file.write_all(data.as_bytes());

        if result.is_err() {
            return Err("Couldn't write ini data to profile (profile creation failed).");
        }

        Ok(profile)
    }
}
