fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:app.manifest");
    }
    tauri_build::build()
}
