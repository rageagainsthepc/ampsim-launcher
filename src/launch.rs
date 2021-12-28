use stable_eyre::{
    eyre::{bail, eyre},
    Report, Result,
};
use std::path::Path;
use std::process::Command;
use std::ptr;
use windows::{
    core::GUID,
    Win32::{
        Foundation::CloseHandle,
        System::{
            Console::FreeConsole,
            Memory::LocalFree,
            Power::{PowerGetActiveScheme, PowerSetActiveScheme},
            Threading::{
                OpenProcess, SetPriorityClass, HIGH_PRIORITY_CLASS, PROCESS_SET_INFORMATION,
            },
        },
    },
};

static HIGH_PERF_PLAN_GUID: &str = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";

fn get_active_power_plan() -> Result<GUID> {
    let mut guid_result: *mut GUID = ptr::null_mut();
    unsafe {
        let success = PowerGetActiveScheme(None, &mut guid_result);
        if success == 0 {
            let active_power_plan_guid = guid_result.read();
            let free_handle = LocalFree(guid_result as _);
            if free_handle != 0 {
                println!("Unable to free guid")
            }
            Ok(active_power_plan_guid)
        } else {
            Err(eyre!(
                "Unable to get active power plan. Error code: {}",
                success
            ))
        }
    }
}

fn switch_power_plan(plan_guid: &GUID) -> Result<()> {
    unsafe {
        let success = PowerSetActiveScheme(None, plan_guid);
        if success != 0 {
            Err(eyre!(
                "Unable to switch to high performance plan. Error code: {}",
                success
            ))
        } else {
            Ok(())
        }
    }
}

fn elevate_process_priority(id: u32) -> Result<()> {
    unsafe {
        let proc_handle = OpenProcess(PROCESS_SET_INFORMATION, false, id);
        if proc_handle.is_invalid() {
            bail!("Unable to open process")
        }

        let success = SetPriorityClass(proc_handle, HIGH_PRIORITY_CLASS);
        if !success.as_bool() {
            bail!("Unable to set priority class");
        }

        // no need to check the return value, there is nothing we can do anyway at this point
        CloseHandle(proc_handle);
    }
    Ok(())
}

fn hide_console_window() {
    unsafe {
        FreeConsole();
    }
}

fn spawn_child_and_wait(cmd: &mut Command) -> Result<()> {
    let mut child = cmd
        .spawn()
        .map_err(|e| Report::from(e).wrap_err("Unable to launch target process"))?;
    // apparently, detaching from the console will prevent spawning child processes
    hide_console_window();
    elevate_process_priority(child.id())?;
    let _ = child.wait();

    Ok(())
}

pub(crate) fn launch(program: &Path) -> Result<()> {
    let active_power_plan_guid = get_active_power_plan()?;

    let high_perf_plan = GUID::try_from(HIGH_PERF_PLAN_GUID).unwrap();
    switch_power_plan(&high_perf_plan)?;

    let mut cmd = Command::new(program);
    match spawn_child_and_wait(&mut cmd) {
        Ok(()) => switch_power_plan(&active_power_plan_guid),
        Err(e) => match switch_power_plan(&active_power_plan_guid) {
            Ok(()) => Err(e),
            Err(e2) => Err(e
                .wrap_err(e2)
                .wrap_err("While processing an error, another error occurred")),
        },
    }
}
