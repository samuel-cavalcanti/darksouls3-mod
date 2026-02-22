use core::ffi::c_void;
use std::ffi::CString;

use windows::{
    Win32::{
        Foundation::HMODULE,
        System::{
            LibraryLoader::{GetProcAddress, LoadLibraryW},
            SystemInformation::GetSystemDirectoryW,
        },
    },
    core::{HRESULT, PCSTR, PCWSTR},
};

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

export_func!(D3DPreprocess,D3D_PREPROCESS,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
pSourceName: *const c_void,
pDefines:*const c_void,
pInclude:*const c_void,
ppCodeText:*mut *mut c_void,
ppErrorMsgs:*mut *mut c_void
);

export_func!(D3DGetDebugInfo,D3D_GET_DEBUG_INFO,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
ppDebugInfo: *mut *mut c_void
);

export_func!(D3DReflect,D3D_REFLECT,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
pInterface:*const c_void,
ppreflector : *mut *mut c_void
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

export_func!(D3DGetInputAndOutputSignatureBlob,D3D_GET_INPUT_AND_OUTPUT_SIGNATURE_BLOB,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
ppSignatureBlob:*mut *mut c_void
);

export_func!(D3DStripShader,D3D_STRIP_SHADER,HRESULT,
pShaderBytecode:*const c_void,
bytecodeLength:usize,
uStripFlags:u32,
ppStrippedBlob:*mut *mut c_void
);

export_func!(D3DGetBlobPart,D3D_GET_BLOB_PART,HRESULT,
pSrcData:*const c_void,
srcDataSize:usize,
part:*const c_void,
flags:u32,
ppart:*mut *mut c_void
);

export_func!(D3DCompressShaders,D3D_COMPRESS_SHADERS,HRESULT,
uNumShaders:u32,
pShaderData:*const c_void,
uflags:u32,
ppCompressedData:*mut *mut c_void
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

export_func!(D3DCreateBlob,D3D_CREATE_BLOB,HRESULT,
size:usize,
ppblob:*mut *mut c_void
);

static mut GENUINE_LIB: Option<HMODULE> = None;
const BACKSLASH: u16 = 0x005C;
const BUFFER_SIZE: usize = 512;

fn load_system32_dll() -> HMODULE {
    let mut system_dir_path: [u16; BUFFER_SIZE] = [0; BUFFER_SIZE];
    unsafe {
        if let Some(dll) = GENUINE_LIB {
            return dll;
        }

        let size_system_dir_path = GetSystemDirectoryW(Some(&mut system_dir_path)) as usize;
        eprintln!(
            "system path: {:?}",
            &system_dir_path[..size_system_dir_path]
        );

        let (dll_path, _path_size) = join_with_module_name(system_dir_path, size_system_dir_path);

        let dll_path = PCWSTR::from_raw(dll_path.as_ptr());
        let path = dll_path.to_string().unwrap();

        let error_mgs = format!("System DLL not found, Path:{path}");

        let dll = LoadLibraryW(dll_path).expect(&error_mgs);

        GENUINE_LIB = Some(dll);

        dll
    }
}

fn join_with_module_name(
    dll_dir_path: [u16; BUFFER_SIZE],
    size_path: usize,
) -> ([u16; BUFFER_SIZE], usize) {
    let mut dll_dir_path = dll_dir_path;
    let module_name = windows::core::w!("D3DCOMPILER_43.dll");
    let size_file_name = unsafe { module_name.len() };

    dll_dir_path[size_path] = BACKSLASH;
    let size_lib_dir = size_path + 1;

    let new_size = size_file_name + size_lib_dir;

    if new_size >= BUFFER_SIZE {
        panic!(
            "Error on load system DLL: new size is bigger that buffer size. new size: {size_lib_dir} + {size_file_name}, buffer size {BUFFER_SIZE}"
        );
    }

    let mut dll_path = dll_dir_path;

    let module_slice = unsafe { std::slice::from_raw_parts(module_name.as_ptr(), size_file_name) };

    dll_path[size_lib_dir..new_size].clone_from_slice(module_slice);

    eprintln!("full path: {:?}", &dll_path[0..new_size]);

    (dll_path, new_size)
}


// D3dcompiler_47.dll functions

// export_func!(D3DAssemble,
// D3D_ASSEMBLE_PTR,
// HRESULT,
// pSrcData: *const c_void,
// SrcDataSize: usize,
// pSourceName: *const c_void,
// pDefines: *const c_void,
// pInclude: *const c_void,
// Flags: u32,
// ppCode: *mut *mut c_void,
// ppErrorMsgs: *mut *mut c_void
// );
//
//
//
