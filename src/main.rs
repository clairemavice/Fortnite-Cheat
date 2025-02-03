mod gui;
mod memory;
mod features;
mod utils;

use gui::Overlay;
use utils::config::Config;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_: usize, call_reason: u32, _: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(move || {
            let config = Config::load();
            let mut overlay = Overlay::new();
            overlay.run(config);
        });
    }
    true
}