use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

pub struct ComInit;
impl ComInit {
    pub fn new() -> Self {
        unsafe {
            CoInitializeEx(Some(std::ptr::null_mut()), COINIT_APARTMENTTHREADED).unwrap();
        }
        ComInit
    }
}
impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
