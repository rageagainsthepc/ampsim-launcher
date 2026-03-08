use std::sync::OnceLock;

use cruet::Inflector;
use windows::{
    Win32::UI::WindowsAndMessaging::{MB_ICONSTOP, MessageBoxW},
    core::HSTRING,
};

static EXE_NAME: OnceLock<String> = OnceLock::new();

pub(crate) fn show(message: &str) {
    let exe_name = EXE_NAME.get_or_init(|| {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.file_stem().map(|s| s.to_os_string()))
            .and_then(|s| s.into_string().ok())
            .unwrap_or_else(|| "AmpSim Launcher".to_string())
            .to_title_case()
    });
    unsafe {
        MessageBoxW(
            None,
            &HSTRING::from(message),
            &HSTRING::from(exe_name),
            MB_ICONSTOP,
        );
    }
}
