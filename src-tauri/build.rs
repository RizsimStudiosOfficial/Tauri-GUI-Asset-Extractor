fn main() {
    #[cfg(target_os = "windows")]
        {
            let windows = tauri_build::WindowsAttributes::new()
                .append_rc_content("LANGUAGE 0x09, 0x01\n");
            let attrs = tauri_build::Attributes::new().windows_attributes(windows);
            tauri_build::try_build(attrs).expect("failed to run build script");
        }

    #[cfg(not(target_os = "windows"))]

    tauri_build::build();
}