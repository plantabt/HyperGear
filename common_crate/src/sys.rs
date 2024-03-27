use std::{env, path::PathBuf};

use windows::{core::PCSTR, Win32::{Foundation::{HMODULE, HWND, MAX_PATH}, System::LibraryLoader::{GetModuleFileNameA, GetModuleHandleA, GetProcAddress, LoadLibraryA}, UI::WindowsAndMessaging::MessageBoxA}};

use crate::{debug_print, utilis::StringToPCSTR};
#[warn(unused_assignments)]

#[allow(non_snake_case)]
pub fn GetCurrentDir()->String{
    let exe_path = env::current_exe().expect("Get current path failed!");
    let dir_path = exe_path.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    dir_path.display().to_string()
}
#[allow(non_snake_case)]
pub fn GetCurrentExeName()->String{
    unsafe{
    let mut filename=[b'\x00';MAX_PATH as usize];
    GetModuleFileNameA(HMODULE(0),&mut filename);
    let fullpath = String::from_utf8_lossy(&filename).trim_matches('\u{0}').to_string();
    GetAppNameFromFullpath(fullpath)
}

}
#[allow(non_snake_case)]
pub fn GetPathFromFullpath(fullpath:String)->String{
    let full_path = PathBuf::from(fullpath.to_owned());
    let dir_path = full_path.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    dir_path.display().to_string()
}

#[allow(non_snake_case)]
pub fn GetAppNameFromFullpath(fullpath:String)->String{
    let full_path = PathBuf::from(fullpath.to_owned());
    let dir_path = full_path.file_name().unwrap().to_str().unwrap();
    dir_path.to_string()
}

#[allow(non_snake_case)]
pub fn OpenDebugWindow(){
    
}


#[allow(non_snake_case)]
pub fn GetFuncAddress(dllname:&str,funcname:&str)->u64{
    unsafe{
        let mut hmod=HMODULE(0);
        debug_print!("GetFuncAddr {} -> {}",dllname,funcname);

        let t_dllname = PCSTR(StringToPCSTR(&dllname.to_string()).into_raw() as *mut u8);
        let t_funcname = PCSTR(StringToPCSTR(&funcname.to_string()).into_raw() as *mut u8);
       // MessageBoxA(HWND(0),t_dllname,t_funcname,windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE(0));
    match GetModuleHandleA(t_dllname){
        Ok(_hmod) => {
            hmod=_hmod;
        },
        Err(e) => {
            debug_print!("GetFuncAddr dll not found1: {},{:?}",dllname,e);
            match LoadLibraryA(t_dllname){
                Ok(_hmod) => hmod=_hmod,
                Err(e) => {
                    debug_print!("GetFuncAddr dll not found2: {},{:?}",dllname,e);
                    return 0
                },
            }
        },
    }
    
    match GetProcAddress(hmod, t_funcname){
        Some(addr) =>{
            //let ret = transmute::<FARPROC,T>(addr);
            return addr as u64;
        },
        None => {
            debug_print!("GetFuncAddr func not found: {}",dllname);
        },
    }

}
    0
}