use crate::error::Error;

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
