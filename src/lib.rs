//!
//! This crate provides easy access to module data.
//!
//! - It supports reading images from disk, using Windows' default search paths.
//! - It also supports simply loading the image into memory and then reading it. This will result in code execution.
//!
//! Please note: these 2 methods DO provide differing results, as addresses and such inside a dynamically loaded image are filled out by the loader.
//!

use std::path::Path;

use winapi::um::{psapi::MODULEINFO, winnt::HANDLE};

///
/// Get a module's memory region from disk.
///
/// # Arguments
/// * `image` - the image name, e.g. `"user32.dll"`
///
pub fn get_static(image: String) -> Option<Vec<u8>> {
    let mut out_dir = std::env::var("OUT_DIR").unwrap_or_default();
    out_dir.push_str("\\game\\");

    // windows search paths
    // cwd -> system32 -> windows -> game/
    let paths = [
        Path::new(&out_dir),
        Path::new(""),
        Path::new("C:\\Windows\\system32\\"),
        Path::new("C:\\Windows\\"),
        Path::new("game\\"),
    ];

    let mut file = paths
        .iter()
        .find(|path| {
            let mut buf = path.to_path_buf();
            buf.push(&image);
            buf.is_file()
        })?
        .to_path_buf();

    file.push(&image);

    std::fs::read(file).ok()
}

///
/// Get a module's memory region from memory.
///
/// # Arguments
/// * `image` - the image name, e.g. `"user32.dll"`
/// * `load_if_necessary` - if the image is not already loaded, load it.
///  
pub fn get_dynamic(image: String, load_if_necessary: bool) -> Option<(*mut u8, usize)> {
    let image = std::ffi::CString::new(image.as_str()).unwrap();
    unsafe {
        let mut module = winapi::um::libloaderapi::GetModuleHandleA(image.as_ptr());

        if module.is_null() && load_if_necessary {
            module = winapi::um::libloaderapi::LoadLibraryA(image.as_ptr());
        }

        if module.is_null() {
            return None;
        }

        let mut moduleinfo: MODULEINFO = std::mem::zeroed();

        let success = winapi::um::psapi::GetModuleInformation(
            usize::MAX as HANDLE,
            module,
            &mut moduleinfo,
            std::mem::size_of::<MODULEINFO>() as u32,
        );

        match success != 0 {
            true => Some((
                moduleinfo.lpBaseOfDll as *mut u8,
                moduleinfo.SizeOfImage as usize,
            )),
            false => None,
        }
    }
}
