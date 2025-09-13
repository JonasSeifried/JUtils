use crate::error::{Error, Result};
use std::collections::HashSet;

use windows::Win32::Media::Audio::ISimpleAudioVolume;
use windows::Win32::System::Com;
use windows::Win32::{
    Foundation::{CloseHandle, MAX_PATH},
    Media::Audio::{
        eRender, IAudioSessionControl, IAudioSessionControl2, IAudioSessionEnumerator,
        IAudioSessionManager2, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
    },
    System::Com::CLSCTX_ALL,
    System::ProcessStatus::GetModuleBaseNameW,
    System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
};
use windows_core::{Interface, GUID};

use crate::windows_common::ComInit;

struct SessionInfo {
    display_name: String,
    exe_name: Option<String>,
}

fn extract_session_info(session_control: &IAudioSessionControl) -> Result<SessionInfo> {
    let session_control2: IAudioSessionControl2 = session_control.cast()?;
    let display_name = unsafe { session_control.GetDisplayName() }?;
    let display_name_str = unsafe { display_name.to_string() }?;

    let process_id = unsafe { session_control2.GetProcessId() }?;
    let exe_name = if process_id != 0 {
        let process_handle = match unsafe {
            OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                false,
                process_id,
            )
        } {
            Ok(handle) if !handle.is_invalid() => handle,
            _res => {
                if let Err(e) = _res {
                    log::warn!("Failed to open process {}: {}", process_id, e);
                }
                return Ok(SessionInfo {
                    display_name: display_name_str,
                    exe_name: None,
                });
            }
        };

        let mut exe_name_buffer: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let length = unsafe { GetModuleBaseNameW(process_handle, None, &mut exe_name_buffer) };
        unsafe { CloseHandle(process_handle) }?;

        if length > 0 {
            Some(String::from_utf16_lossy(
                &exe_name_buffer[..length as usize],
            ))
        } else {
            None
        }
    } else {
        None
    };

    Ok(SessionInfo {
        display_name: display_name_str,
        exe_name,
    })
}

pub fn get_running_apps_with_audio_sessions() -> Result<Vec<String>> {
    let _com_init = ComInit::new();
    let mut app_names: HashSet<String> = HashSet::new();

    unsafe {
        let device_enumerator: IMMDeviceEnumerator =
            Com::CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        let devices = device_enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
        let device_count = devices.GetCount()?;

        for i in 0..device_count {
            let device = devices.Item(i)?;
            let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)?;
            let session_enumerator: IAudioSessionEnumerator =
                session_manager.GetSessionEnumerator()?;
            let session_count = session_enumerator.GetCount()?;

            for j in 0..session_count {
                let session_control: IAudioSessionControl = session_enumerator.GetSession(j)?;
                let info = extract_session_info(&session_control)?;

                if !info.display_name.trim().is_empty() {
                    app_names.insert(info.display_name);
                } else if let Some(exe_name) = info.exe_name {
                    app_names.insert(exe_name);
                }
            }
        }
        log::debug!("Found running apps: {:?}", app_names);
        Ok(app_names.into_iter().collect())
    }
}

pub fn mute_app_by_name(app_name: &str) -> Result<()> {
    let _com_init = ComInit::new();
    let mut muted = false;

    unsafe {
        let device_enumerator: IMMDeviceEnumerator =
            Com::CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        let devices = device_enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
        let device_count = devices.GetCount()?;

        for i in 0..device_count {
            let device = devices.Item(i)?;
            let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)?;
            let session_enumerator: IAudioSessionEnumerator =
                session_manager.GetSessionEnumerator()?;
            let session_count = session_enumerator.GetCount()?;

            for j in 0..session_count {
                let session_control: IAudioSessionControl = session_enumerator.GetSession(j)?;
                let info = extract_session_info(&session_control)?;

                if info.display_name.eq_ignore_ascii_case(app_name)
                    || info
                        .exe_name
                        .as_deref()
                        .map_or(false, |exe| exe.eq_ignore_ascii_case(app_name))
                {
                    log::info!("Muting session for '{}'", app_name);
                    mute_session(&session_control)?;
                    muted = true;
                }
            }
        }
    }
    if muted {
        Ok(())
    } else {
        Err(Error::UnexpectedError("Element not found".to_string()))
    }
}

fn mute_session(session_control: &IAudioSessionControl) -> Result<()> {
    let simple_audio_volume: ISimpleAudioVolume = session_control.cast()?;
    let mute: bool = unsafe { simple_audio_volume.GetMute()?.into() };
    log::info!("Current mute state: {}", mute);
    unsafe { simple_audio_volume.SetMute(!mute, &GUID::new()?)? };
    Ok(())
}
