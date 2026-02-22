#![windows_subsystem = "windows"]

use core::ffi::c_void;
use std::{ffi::CString, usize};

use windows::{
    Win32::{
        Foundation::{HINSTANCE, HMODULE},
        System::{
            LibraryLoader::{GetProcAddress, LoadLibraryW},
            SystemInformation::GetSystemDirectoryW,
        },
    },
    core::{BOOL, HRESULT, PCSTR, PCWSTR},
};

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

macro_rules! export_func {
    ($name:ident, $fn_ptr:ident, $return:ty, $($params:ident: $types:ty),*) => {

        #[allow(non_snake_case, improper_ctypes)]
        type $name = unsafe extern "system" fn($($params: $types),*) -> $return;
        static mut $fn_ptr: Option<$name> = None;

        #[unsafe(no_mangle)]
        #[allow(non_snake_case, improper_ctypes)]
/// # Safety
/// it's not safe.
        pub unsafe extern "system" fn $name($($params: $types),*) -> $return {
            let function_name = stringify!($name);
            unsafe{

                eprintln!("Mod-DarkSouls3:access:{function_name}");
                if let Some(fun) = $fn_ptr{
                    return fun($($params),*);
                }

                let module = load_system32_dll();
                let c_string = CString::new(stringify!($name)).expect(&format!("Unable do crate Cstring: {function_name}"));
                let func = GetProcAddress(module,PCSTR(c_string.as_ptr() as *const u8)).expect(concat!(stringify!($name), " not Founded"));
                let func:$name = std::mem::transmute(func);
                $fn_ptr = Some(func);

                func($($params),*)
            }
        }
    };
}

static mut GENUINE_LIB: Option<HMODULE> = None;
const BACKSLASH: u16 = 0x005C;

fn load_system32_dll() -> HMODULE {
    let mut genuine_lib_dir: [u16; 512] = [0; 512];
    unsafe {
        if let Some(dll) = GENUINE_LIB {
            return dll;
        }

        let size_lib_dir = GetSystemDirectoryW(Some(&mut genuine_lib_dir)) as usize;

        let dll_path = join_with_module_name(genuine_lib_dir, size_lib_dir);

        let path = dll_path.to_string().unwrap();

        let error_mgs = format!("System DLL not found, Path:{path}");

        let dll = LoadLibraryW(dll_path).expect(&error_mgs);

        GENUINE_LIB = Some(dll);

        dll
    }
}

fn join_with_module_name(dll_dir_path: [u16; 512], size_path: usize) -> PCWSTR {
    let mut genuine_lib_dir = dll_dir_path;
    let module_name = windows::core::w!("D3DCOMPILER_43.dll");
    let size_file_name = unsafe { module_name.len() };

    genuine_lib_dir[size_path] = BACKSLASH;
    let size_lib_dir = size_path + 1;

    if size_file_name + size_lib_dir >= 512 {
        panic!("Error on load system DLL: size_file_name + size_lib_dir => 512")
    }

    let mut dll_path = dll_dir_path;

    let module_slice = unsafe { std::slice::from_raw_parts(module_name.0, size_file_name) };

    dll_path[size_lib_dir..(size_file_name + size_lib_dir)].clone_from_slice(module_slice);
    let dll_path = dll_path[0..(size_lib_dir + size_file_name)].as_ptr();

    PCWSTR::from_raw(dll_path)
}

export_func!(D3DAssemble,
D3D_ASSEMBLE_PTR,
HRESULT,
pSrcData: *const c_void,
SrcDataSize: usize,
pSourceName: *const c_void,
pDefines: *const c_void,
pInclude: *const c_void,
Flags: u32,
ppCode: *mut *mut c_void,
ppErrorMsgs: *mut *mut c_void
);

export_func!(DebugSetMute, DEBUG_SET_MUTE, (),);
export_func!(D3DCompile,D3D_COMPILE,HRESULT,
pSrcData:*const c_void,
SrcDataSize:usize,
pSourceName:*const c_void,
pDefines: *const c_void ,
pInclude:*const c_void,
pEntrypoint:*const c_void,
pTarget:*const c_void,
flags1:u32,
flags2:u32,
ppCode:*mut *mut c_void,
ppErrorMsgs:*mut *mut c_void
);

export_func!(D3DCompressShaders,D3D_COMPRESS_SHADERS,HRESULT,
uNumShaders:u32,
pShaderData:*const c_void,
uflags:u32,
ppCompressedData:*mut *mut c_void
);

export_func!(D3DCreateBlob,D3D_CREATE_BLOB,HRESULT,
size:usize,
ppblob:*mut *mut c_void
);

export_func!(D3DDecompressShaders,D3D_DECOMPRESS_SHADERS,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
uNumShaders:u32,
uStartIndex:u32,
pIndices:*const u32,
uFlags:u32,
ppShaders:*mut *mut c_void,
pTotalShaders:u32
);

export_func!(D3DDisassemble,D3D_DISASSEMBLE,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
uflags:u32,
szComments:*const c_void,
ppDisassembly:*mut *mut c_void
);

export_func!(D3DDisassemble10Effect,D3D_DISASSEMBLE10_EFFECT,HRESULT,
pEffect:*const c_void,
enableColorCode:bool,
ppDisassembly:*mut *mut c_void
);

export_func!(D3DGetBlobPart,D3D_GET_BLOB_PART,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
part:*const c_void,
flags:u32,
ppart:*mut *mut c_void
);

export_func!(D3DGetDebugInfo,D3D_GET_DEBUG_INFO,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
ppDebugInfo: *mut *mut c_void
);

export_func!(D3DGetInputAndOutputSignatureBlob,D3D_GET_INPUT_AND_OUTPUT_SIGNATURE_BLOB,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
ppSignatureBlob:*mut *mut c_void
);

export_func!(D3DGetInputSignatureBlob,D3D_GET_INPUT_SIGNATURE_BLOB,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
ppSignatureBlob:*mut *mut c_void
);

export_func!(D3DGetOutputSignatureBlob,D3D_GET_OUTPUT_SIGNATURE_BLOB,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
ppSignatureBlob:*mut *mut c_void
);

export_func!(D3DPreprocess,D3D_PREPROCESS,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
pSourceName: *const c_void,
pDefines:*const c_void,
pInclude:*const c_void,
ppCodeText:*mut *mut c_void,
ppErrorMsgs:*mut *mut c_void
);

export_func!(D3DReflect,D3D_REFLECT,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
pInterface:*const c_void,
ppreflector : *mut *mut c_void
);

export_func!(D3DReturnFailure1,DED_RETURN_FAILURE1,HRESULT,
    args:*const c_void
);
// export_func!(D3DStripShader);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
