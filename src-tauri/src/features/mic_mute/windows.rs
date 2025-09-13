use crate::error::Result;
use crate::windows_common::ComInit;

pub fn toggle_mic() -> Result<bool> {
    use windows::{
        core::GUID,
        Win32::{
            Media::Audio::{
                eCapture, eCommunications, Endpoints::IAudioEndpointVolume, IMMDeviceEnumerator,
                MMDeviceEnumerator,
            },
            System::Com::{CoCreateInstance, CLSCTX_ALL, CLSCTX_INPROC_SERVER},
        },
    };
    unsafe {
        let _com_init = ComInit::new();
        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = IMMDeviceEnumerator::GetDefaultAudioEndpoint(
            &device_enumerator,
            eCapture,
            eCommunications,
        )?;

        let iae: IAudioEndpointVolume = device.Activate(CLSCTX_INPROC_SERVER, None)?;
        let new_state: bool = !iae.GetMute()?.as_bool();
        iae.SetMute(new_state, &GUID::default())?;
        Ok(new_state)
    }
}