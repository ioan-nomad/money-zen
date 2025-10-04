fn main() {
    // Skip Windows resource file generation
    #[cfg(target_os = "windows")]
    std::env::set_var("TAURI_SKIP_WINRES", "true");

    tauri_build::build()
}