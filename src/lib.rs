#![windows_subsystem = "windows"]


/// D3D Proxy, proxy all functions of "d3dcompiler_43.dll 
/// https://git.redump.net/mame/tree/3rdparty/dxsdk/Include/d3dcompiler.h?id=06b848185fb4559750a0f4a8de8a5a7789a9eca5
mod d3d_proxy;
pub use d3d_proxy::*;

use windows::{Win32::Foundation::HINSTANCE, core::BOOL};

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
/// # Safety
/// it's not safe.
pub unsafe extern "system" fn DllMain(
    _base_addr: HINSTANCE,
    reason: u32,
    _isstatic: BOOL,
) -> windows::core::BOOL {
    if reason == windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
        load_ini_file();
        apply_mod();
    }

    true.into()
}

fn load_ini_file() {}

fn apply_mod() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
