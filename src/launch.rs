use camino::Utf8Path;
use stable_eyre::{Report, Result, eyre::eyre};
use std::ptr;
use std::{ffi::c_void, process::Command};
use windows::{
    Win32::{
        Foundation::{CloseHandle, HLOCAL, LocalFree},
        System::{
            Console::FreeConsole,
            Power::{PowerGetActiveScheme, PowerSetActiveScheme},
            Threading::{
                GetCurrentProcess, HIGH_PRIORITY_CLASS, OpenProcess, PROCESS_MODE_BACKGROUND_BEGIN,
                PROCESS_MODE_BACKGROUND_END, PROCESS_SET_INFORMATION, SetPriorityClass,
            },
        },
    },
    core::GUID,
};

static HIGH_PERF_PLAN_GUID: &str = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";

fn get_active_power_plan() -> Result<GUID> {
    let mut guid_result: *mut GUID = ptr::null_mut();
    unsafe {
        let success = PowerGetActiveScheme(None, &mut guid_result);
        if success.is_ok() {
            let active_power_plan_guid = guid_result.read();
            let free_handle = LocalFree(Some(HLOCAL(guid_result as *mut c_void)));
            if !free_handle.is_invalid() {
                println!("Unable to free guid")
            }
            Ok(active_power_plan_guid)
        } else {
            Err(eyre!(
                "Unable to get active power plan. Error code: {}",
                success.0
            ))
        }
    }
}

fn switch_power_plan(plan_guid: &GUID) -> Result<()> {
    unsafe {
        let success = PowerSetActiveScheme(None, Some(plan_guid));
        if success.is_err() {
            Err(eyre!(
                "Unable to switch to high performance plan. Error code: {}",
                success.0
            ))
        } else {
            Ok(())
        }
    }
}

fn elevate_process_priority(id: u32) -> Result<()> {
    unsafe {
        match OpenProcess(PROCESS_SET_INFORMATION, false, id) {
            Ok(proc_handle) => {
                let success = SetPriorityClass(proc_handle, HIGH_PRIORITY_CLASS);
                success.map_err(|e| eyre!(e.message()).wrap_err("Unable to set priority class"))?;

                // no need to check the return value, there is nothing we can do anyway at this point
                CloseHandle(proc_handle)?;
                Ok(())
            }
            Err(e) => Err(eyre!(e.message()).wrap_err("Unable to open process")),
        }
    }
}

fn hide_console_window() {
    unsafe {
        let _ = FreeConsole();
    }
}

fn begin_background_mode() -> Result<()> {
    unsafe {
        let success = SetPriorityClass(GetCurrentProcess(), PROCESS_MODE_BACKGROUND_BEGIN);
        success.map_err(|e| eyre!(e.message()).wrap_err("Unable to begin background mode"))
    }
}

fn end_background_mode() -> Result<()> {
    unsafe {
        let success = SetPriorityClass(GetCurrentProcess(), PROCESS_MODE_BACKGROUND_END);
        success.map_err(|e| eyre!(e.message()).wrap_err("Unable to end background mode"))
    }
}

fn spawn_child_and_wait(cmd: &mut Command, no_console: bool) -> Result<()> {
    let mut child = cmd
        .spawn()
        .map_err(|e| Report::from(e).wrap_err("Unable to launch target process"))?;
    // apparently, detaching from the console will prevent spawning child processes
    if no_console {
        hide_console_window();
    }
    elevate_process_priority(child.id())?;

    begin_background_mode()?;
    let _ = child.wait();
    end_background_mode()?;

    Ok(())
}

pub(crate) fn launch(program: &Utf8Path, no_console: bool) -> Result<()> {
    let active_power_plan_guid = get_active_power_plan()?;

    let high_perf_plan = GUID::try_from(HIGH_PERF_PLAN_GUID).unwrap();
    switch_power_plan(&high_perf_plan)?;

    let mut cmd = Command::new(program);
    match spawn_child_and_wait(&mut cmd, no_console) {
        Ok(()) => switch_power_plan(&active_power_plan_guid),
        Err(e) => match switch_power_plan(&active_power_plan_guid) {
            Ok(()) => Err(e),
            Err(e2) => Err(e
                .wrap_err(e2)
                .wrap_err("While processing an error, another error occurred")),
        },
    }
}
