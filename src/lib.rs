#![windows_subsystem = "windows"]

/// D3D Proxy, proxy all functions of "d3dcompiler_43.dll
/// https://git.redump.net/mame/tree/3rdparty/dxsdk/Include/d3dcompiler.h?id=06b848185fb4559750a0f4a8de8a5a7789a9eca5
/// win-sdk-headers/d3dcompiler.h
use std::sync::OnceLock;

use ntapi::ntmmapi::{NtReadVirtualMemory, NtWriteVirtualMemory};
use windows::{
    Win32::{
        Foundation::{HANDLE, HINSTANCE, HMODULE, HWND},
        System::{
            LibraryLoader::GetModuleHandleA,
            Threading::{OpenProcess, PROCESS_ALL_ACCESS},
        },
        UI::WindowsAndMessaging::{
            FindWindowA, GWL_EXSTYLE, GWL_STYLE, GetSystemMetrics, GetWindowLongPtrW,
            GetWindowRect, GetWindowThreadProcessId, SM_CXSCREEN, SM_CYSCREEN, SWP_ASYNCWINDOWPOS,
            SetWindowLongPtrW, SetWindowPos, WS_EX_TOPMOST, WS_POPUP, WS_VISIBLE,
        },
    },
    core::{BOOL, PCSTR},
};

mod d3d_proxy;
pub use d3d_proxy::*;

use crate::mod_config::ModConfig;

mod mod_config;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
/// # Safety
/// it's not safe.
pub unsafe extern "system" fn DllMain(
    _base_addr: HINSTANCE,
    reason: u32,
    _isstatic: BOOL,
) -> windows::core::BOOL {
    eprintln!(
        "load DDL MAIN: {reason}, PROCESS ATTACH number: {}",
        windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH
    );

    if reason == windows::Win32::System::SystemServices::DLL_THREAD_ATTACH {
        // try_to_load_mod();
    }

    true.into()
}

static mut MOD_LOADED: bool = false;
pub fn try_to_load_mod() {
    static INI_CONFIG: OnceLock<ModConfig> = OnceLock::new();
    let config = INI_CONFIG.get_or_init(ModConfig::load_ini_file);

    unsafe {
        if !MOD_LOADED {
            if let Err(e) = apply_mod(config) {
                eprintln!("Unable to apply mod {e:?}");
            } else {
                eprintln!("MOD-DARKSOULS-3:MOD LOADED");
                MOD_LOADED = true;
            }
        }
    }
}

unsafe fn apply_mod(config: &ModConfig) -> Result<(), windows::core::Error> {
    // std::thread::sleep(Duration::from_secs(2));
    unsafe {
        let window = FindWindowA(PCSTR::null(), windows::core::s!("DARK SOULS III"))?;
        let base_address = GetModuleHandleA(PCSTR::null())?;
        let mut pid = 0;
        GetWindowThreadProcessId(window, Some(&mut pid));

        let p_handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid)?;

        change_fps_cap(config.fps, p_handle, base_address)?;

        change_window_border(window, config.enable_borderless)?;
        change_window_pos(window, config.center_x)?;
        skip_intro(config.skip_intro, base_address)?;
    }

    Ok(())
}

unsafe fn skip_intro(skip: bool, base_address: HMODULE) -> Result<(), windows::core::Error> {
    let _base_addr = base_address;

    if !skip {
        return Ok(());
    }

    //TODO: how to change memory to skip the intro ?

    // let status_w2 = NtWriteVirtualMemory(
    //     p_handle.0 as *mut ntapi::winapi::ctypes::c_void,
    //     (gfx_address + 0x358) as *mut ntapi::winapi::ctypes::c_void,
    //     p_u8 as *mut ntapi::winapi::ctypes::c_void,
    //     size_of::<u8>(),
    //     std::ptr::null_mut(),
    // );

    Ok(())
}

unsafe fn change_window_border(
    window: HWND,
    enable_borderless: bool,
) -> Result<(), windows::core::Error> {
    if !enable_borderless {
        return Ok(());
    }

    unsafe {
        let current_style = GetWindowLongPtrW(window, GWL_EXSTYLE);
        SetWindowLongPtrW(
            window,
            GWL_EXSTYLE,
            current_style | (WS_EX_TOPMOST.0 as isize),
        );
        SetWindowLongPtrW(
            window,
            GWL_STYLE,
            (WS_POPUP.0 as isize) | (WS_VISIBLE.0 as isize),
        );
    }

    Ok(())
}

unsafe fn change_fps_cap(
    new_fps: f32,
    p_handle: HANDLE,
    base_address: HMODULE,
) -> Result<(), windows::core::Error> {
    unsafe {
        let base_address = base_address.0 as u64;
        let graphics_address = base_address + 0x489DD10;
        let mut gfx_address: u64 = 0;
        let mut debug_fps = 1;
        let mut fps_cap: f32 = new_fps;
        let p_f32: *mut f32 = &mut fps_cap;
        let p_u8: *mut u8 = &mut debug_fps;
        let p_u64: *mut u64 = &mut gfx_address;

        //
        //struct Graphics{
        //  ...
        //  gfx: GFX
        //  ...
        //  }
        //  struct GFx{
        //  ...
        //  debug_fps_cap:f32
        //  dubug:bool
        //
        //  ...
        //  }
        // graphics->gfx ==>base_address + 0x489DD10
        // gfx->debug_fps_cap ==  gfx_address + 0x354
        // gtx->debug == gfx_address + 0x358

        let status_r = NtReadVirtualMemory(
            p_handle.0 as *mut ntapi::winapi::ctypes::c_void,
            graphics_address as *mut ntapi::winapi::ctypes::c_void,
            p_u64 as *mut ntapi::winapi::ctypes::c_void,
            size_of::<u64>(),
            std::ptr::null_mut(),
        );

        assert!(gfx_address != 0);

        let status_w1 = NtWriteVirtualMemory(
            p_handle.0 as *mut ntapi::winapi::ctypes::c_void,
            (gfx_address + 0x354) as *mut ntapi::winapi::ctypes::c_void,
            p_f32 as *mut ntapi::winapi::ctypes::c_void,
            size_of::<f32>(),
            std::ptr::null_mut(),
        );

        let status_w2 = NtWriteVirtualMemory(
            p_handle.0 as *mut ntapi::winapi::ctypes::c_void,
            (gfx_address + 0x358) as *mut ntapi::winapi::ctypes::c_void,
            p_u8 as *mut ntapi::winapi::ctypes::c_void,
            size_of::<u8>(),
            std::ptr::null_mut(),
        );

        let status = [status_r, status_w1, status_w2];

        eprintln!("FPS cap change was Applied: {status:?} SUCCESS==0")
    }

    Ok(())
}

fn change_window_pos(window: HWND, center_x: bool) -> Result<(), windows::core::Error> {
    if !center_x {
        return Ok(());
    }

    unsafe {
        let x_screen_size = GetSystemMetrics(SM_CXSCREEN);
        let _ = GetSystemMetrics(SM_CYSCREEN);

        let mut window_rect = windows::Win32::Foundation::RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        GetWindowRect(window, &mut window_rect)?;

        eprintln!("window rect: {window_rect:?} screen size: {x_screen_size} window: {window:?}");

        let width = window_rect.right - window_rect.left;
        let height = window_rect.bottom - window_rect.top;

        SetWindowPos(
            window,
            None,
            (x_screen_size - width) / 2,
            0,
            width,
            height,
            SWP_ASYNCWINDOWPOS,
        )?;
    }

    Ok(())
}
