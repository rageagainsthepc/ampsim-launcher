use inflector::Inflector;
use lazy_static::lazy_static;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP};

lazy_static! {
    static ref EXE_NAME: String = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
        .unwrap_or_else(|| "AmpSim Launcher".to_string())
        .to_title_case();
}

pub(crate) fn show(message: &str) {
    unsafe {
        MessageBoxW(None, message, EXE_NAME.as_str(), MB_ICONSTOP);
    }
}
