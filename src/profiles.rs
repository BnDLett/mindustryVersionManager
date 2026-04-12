use std::collections::HashMap;
use std::fs::{create_dir, create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use ini::ini;
use crate::apply_attrib;
use crate::utils::{copy_tree, get_index};
use const_format::concatcp;

apply_attrib! {
    #![cfg(target_os = "linux")]

    const MINDUSTRY_PATH: &str = "~/.local/share/Mindustry/";
}

const PROFILE_CONFIG: &str = concatcp!(MINDUSTRY_PATH, "/PROFILE_DATA.ini");
const CONFIG_PATH: &str = concatcp!(MINDUSTRY_PATH, "/../mind_ver");
const PROFILES_PATH: &str = concatcp!(CONFIG_PATH, "/profiles");

// #[cfg(target_os = "windows", target_os = "android")]
// compile_error!("The selected OS is not supported.");

pub struct ProfileManager {
    profiles: Vec<Profile>,
    current_profile: Option<Profile>,
    ini: HashMap<String, HashMap<String, Option<String>>>
}

impl ProfileManager {
    pub fn new() -> Result<ProfileManager, String> {
        create_dir_all(PROFILES_PATH).unwrap();

        let mut manager = ProfileManager{
            profiles: Vec::new(),
            current_profile: None,
            ini: HashMap::new()
        };
        manager.load_ini().unwrap();
        manager.add_current().unwrap();
        Ok(manager)
    }

    pub fn switch_to(&self, profile: &Profile) -> Result<(), &'static str> {
        let current_profile = self.current_profile.clone().unwrap();

        if self.profiles.iter().find(|x| {x.name == profile.name}).is_none() {
            return Err("Profile is not loaded.");
        } else if profile.name == current_profile.name {
            return Ok(());
        }

        let target_path = &*format!("{}/{}", PROFILES_PATH, profile.name);
        let target_dir = Path::new(target_path);
        if !target_dir.exists() || !target_dir.is_dir() {
            return Err("The specified file path cannot be found.")
        }

        let new_path = &*format!("{}/{}", PROFILES_PATH, current_profile.name);
        let new_dir = Path::new(new_path);
        if new_dir.exists() {
            return Err("A profile of the same name already exists (duplicate profile).");
        }

        copy_tree(MINDUSTRY_PATH, new_dir).unwrap();
        remove_dir_all(MINDUSTRY_PATH).unwrap();
        copy_tree(target_dir, MINDUSTRY_PATH).unwrap();
        remove_dir_all(target_dir).unwrap();

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
        if self.current_profile.clone()?.name == name { return self.current_profile.as_ref() }
        
        self.profiles.iter().find(|x| {
            if x.name == name {
                return true;
            }
            
            false
        })
    }

    fn load_ini(&mut self) -> Result<(), ()> {
        let mut data = ini!(safe PROFILE_CONFIG);

        if data.is_err() {
            ProfileManager::generate_ini()?;
            data = ini!(safe PROFILE_CONFIG);
        }

        self.ini = data.unwrap();   // shouldn't be an error. If it still is, then something went
                                    // horribly wrong.
        Ok(())
    }

    fn add_current(&mut self) -> Result<(), ()> {
        let name = self.ini["profile"]["name"].clone();
        if name.is_none() { return Err(()) };

        let profile = Profile::new(&*name.unwrap());
        self.profiles.push(profile);

        Ok(())
    }

    fn generate_ini() -> Result<(), ()> {
        let mut file = File::create(PROFILE_CONFIG).unwrap();

        let time = SystemTime::now().duration_since(UNIX_EPOCH);
        let mut seconds = 0u64;
        if time.is_ok() {
            seconds = time.unwrap().as_secs();
        }

        let name = format!("data-{}.ini", seconds);
        let data = format!("[profile]\nname=data-{}", name);
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

        if path.exists() {
            return Err("Profile already exists.");
        }

        create_dir(path).unwrap();

        Ok(profile)
    }
}
