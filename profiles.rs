apply_attrib! {
    #![cfg(target_os = "linux")]

    const CONFIG_PATH: &str = "/etc/mind-ver";
    const MINDUSTRY_PATH: &str = "/etc/mindustry";
}

#[cfg(target_os = "windows", target_os = "android")]
compile_error!("The selected OS is not supported.");
