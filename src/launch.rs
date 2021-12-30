use camino::Utf8Path;
use defer::defer;
use stable_eyre::{eyre::eyre, Report, Result};
use std::alloc::{GlobalAlloc, System};
use std::mem::align_of;
use std::{alloc::Layout, process::Command};
use std::{ptr, slice};
use windows::Win32::Security::TOKEN_PRIVILEGES;
use windows::{
    core::GUID,
    Win32::{
        Foundation::{CloseHandle, ERROR_INSUFFICIENT_BUFFER},
        Security::{GetTokenInformation, LookupPrivilegeValueA, TokenPrivileges, TOKEN_QUERY},
        System::{
            Console::FreeConsole,
            Memory::LocalFree,
            Power::{PowerGetActiveScheme, PowerSetActiveScheme},
            Threading::{
                GetCurrentProcess, OpenProcess, OpenProcessToken, SetPriorityClass,
                PROCESS_MODE_BACKGROUND_BEGIN, PROCESS_MODE_BACKGROUND_END,
                PROCESS_SET_INFORMATION, REALTIME_PRIORITY_CLASS,
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
        proc_handle
            .ok()
            .map_err(|e| eyre!(e.message()).wrap_err("Unable to open process"))?;

        let success = SetPriorityClass(proc_handle, REALTIME_PRIORITY_CLASS);
        success
            .ok()
            .map_err(|e| eyre!(e.message()).wrap_err("Unable to set priority class"))?;

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

fn begin_background_mode() -> Result<()> {
    unsafe {
        let success = SetPriorityClass(GetCurrentProcess(), PROCESS_MODE_BACKGROUND_BEGIN);
        success
            .ok()
            .map_err(|e| eyre!(e.message()).wrap_err("Unable to begin background mode"))
    }
}

fn end_background_mode() -> Result<()> {
    unsafe {
        let success = SetPriorityClass(GetCurrentProcess(), PROCESS_MODE_BACKGROUND_END);
        success
            .ok()
            .map_err(|e| eyre!(e.message()).wrap_err("Unable to end background mode"))
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

fn check_prio_priv() -> Result<bool> {
    let mut token_handle = Default::default();
    let result;
    let mut luid = Default::default();
    unsafe {
        LookupPrivilegeValueA(None, "SeIncreaseBasePriorityPrivilege", &mut luid)
            .ok()
            .unwrap();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle)
            .ok()
            .unwrap();
        let _close_handle = defer(|| {
            CloseHandle(token_handle);
        });
        let mut buffer_size = 0;
        GetTokenInformation(
            token_handle,
            TokenPrivileges,
            ptr::null_mut(),
            0,
            &mut buffer_size,
        )
        .ok()
        .or_else(|e| match e.win32_error() {
            Some(code) if code == ERROR_INSUFFICIENT_BUFFER => Ok(()),
            _ => Err(eyre!(e.message())
                .wrap_err("Unable to determine buffer size for retrieving token information")),
        })?;
        let token_privileges_buffer_layout = Layout::from_size_align(
            buffer_size.try_into().unwrap(),
            align_of::<TOKEN_PRIVILEGES>(),
        )
        .unwrap();
        let token_privileges_buffer: *mut TOKEN_PRIVILEGES =
            System.alloc(token_privileges_buffer_layout) as _;
        let _dealloc = defer(|| {
            System.dealloc(token_privileges_buffer as _, token_privileges_buffer_layout);
        });
        GetTokenInformation(
            token_handle,
            TokenPrivileges,
            token_privileges_buffer as _,
            buffer_size,
            &mut buffer_size,
        )
        .ok()
        .map_err(|e| eyre!(e.message()).wrap_err("Unable to query token information"))?;
        let luid_and_attributes_slice = slice::from_raw_parts(
            (*token_privileges_buffer).Privileges.as_ptr(),
            (*token_privileges_buffer)
                .PrivilegeCount
                .try_into()
                .unwrap(),
        );
        result = luid_and_attributes_slice.iter().any(|l| l.Luid.eq(&luid));
    }
    Ok(result)
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
