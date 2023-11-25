use crate::error::Error;

#[cfg(target_os = "windows")]
use windows::{
    core::GUID,
    Win32::{
        Media::Audio::{
            eCapture, eCommunications, Endpoints::IAudioEndpointVolume, IMMDeviceEnumerator,
            MMDeviceEnumerator,
        },
        System::Com::{
            CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL, CLSCTX_INPROC_SERVER,
        },
    },
};

#[cfg(target_os = "windows")]
pub fn toggle_mic(mute: bool) -> Result<(), Error> {
    unsafe {
        let _ = CoInitialize(None)?;
        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = IMMDeviceEnumerator::GetDefaultAudioEndpoint(
            &device_enumerator,
            eCapture,
            eCommunications,
        )?;

        let iae: IAudioEndpointVolume = device.Activate(CLSCTX_INPROC_SERVER, None)?;
        iae.SetMute(mute, &GUID::default())?;
        CoUninitialize();
    }

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn toggle_mic(mute: bool) -> Result<(), Error> {
    Ok(())
}
