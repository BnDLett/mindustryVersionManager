use std::{fs, io};
use std::path::Path;
use const_format::concatcp;

crate::apply_attrib! {
    #![cfg(target_os = "linux")]

    pub const MINDUSTRY_PATH: &str = "~/.local/share/Mindustry/";
}

pub const PROFILE_CONFIG: &str = concatcp!(MINDUSTRY_PATH, "/PROFILE_DATA.ini");
pub const CONFIG_PATH: &str = concatcp!(MINDUSTRY_PATH, "/../mind_ver");
pub const PROFILES_PATH: &str = concatcp!(CONFIG_PATH, "/profiles");

// https://stackoverflow.com/a/77835585
#[macro_export]
macro_rules! apply_attrib {
    { #!$attr:tt $($it:item)* } => {
        $(
            #$attr
            $it
        )*
    }
}

pub(crate) fn get_index<T>(vec: &Vec<T>, func: impl Fn(&T) -> bool) -> Result<usize, ()> {
    let mut index = 0usize;
    for item in vec.iter() {
        if func(item) { return Ok(index); };
        index += 1;
    }

    Err(())
}

// https://stackoverflow.com/a/65192210
pub(crate) fn copy_tree(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_tree(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
