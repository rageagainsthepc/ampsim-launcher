use stable_eyre::{eyre::bail, eyre::eyre, Result};
use std::path::Path;
use std::process::Command;
use std::ptr;
use windows::{
    core::GUID,
    Win32::{
        Foundation::CloseHandle,
        System::{
            Memory::LocalFree,
            Power::{PowerGetActiveScheme, PowerSetActiveScheme},
            Registry::HKEY,
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
        let success = PowerGetActiveScheme(HKEY::default(), &mut guid_result);
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
        let success = PowerSetActiveScheme(HKEY::default(), plan_guid);
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

pub(crate) fn launch(program: &Path) -> Result<()> {
    let active_power_plan_guid = get_active_power_plan()?;

    let high_perf_plan = GUID::try_from(HIGH_PERF_PLAN_GUID).unwrap();
    switch_power_plan(&high_perf_plan)?;

    let mut child = Command::new(program).spawn()?;
    elevate_process_priority(child.id())?;
    let _ = child.wait();

    switch_power_plan(&active_power_plan_guid)?;

    Ok(())
}
