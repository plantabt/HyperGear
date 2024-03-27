
use std::{ffi::c_void, fs::OpenOptions, ptr::{self}};

use memmap2::MmapMut;
use windows::Win32::{Foundation::{HANDLE, MAX_PATH}, Storage::FileSystem::GetTempPathA, System::Memory::{MapViewOfFile, VirtualProtect, FILE_MAP_ALL_ACCESS, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS}};

use crate::{debug_print, utilis::make_md5_string};
#[allow(non_snake_case)]
pub fn GetTempPath()->String{
    unsafe{
        let mut tmpbuff =[b'\x00';MAX_PATH as usize];
        GetTempPathA(Some(&mut tmpbuff));
        String::from_utf8_lossy(&tmpbuff).trim_matches('\u{0}').to_string()
}
}
#[allow(non_snake_case)]
pub fn CreateSharedMem(memname:String,memsize:u64)->Result<MmapMut,std::io::Error> {
   unsafe {
        let md5memname = make_md5_string(memname.clone());
        let mut tmppath = GetTempPath();
        tmppath.push_str(&md5memname);
        match std::fs::remove_file(tmppath.clone()){
            Ok(_)=>{
                debug_print!("CreateSharedMem delete: {}",tmppath.clone());
            },
            Err(_)=>{

            }
        }
        match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(tmppath.clone()){
            Ok(hfile)=>{
                hfile.set_len(memsize).unwrap();
                
                debug_print!("CreateSharedMem [o] : {}",tmppath);
                MmapMut::map_mut(&hfile)
            },
            Err(e)=>{
                debug_print!("CreateSharedMem [x] : {}",md5memname);
                Err(e)
            }
        }
    }
}
#[allow(non_snake_case)]
pub fn OpenSharedMem(memname:String)->Result<MmapMut,std::io::Error>  {
    unsafe{
        let md5memname = make_md5_string(memname.clone());
        let mut tmppath = GetTempPath();
        tmppath.push_str(&md5memname);
        match OpenOptions::new()
            .read(true)
            .write(true)
            .open(tmppath.clone()){
                Ok(hfile)=>{
                   // debug_print!("OpenSharedMem [o] : {}",tmppath);
                    MmapMut::map_mut(&hfile)
                },
                Err(e)=>{
                    debug_print!("OpenSharedMem [x] : {}",md5memname);
                    Err(e)
                }
            }
}
}

#[allow(non_snake_case)]
#[warn(dropping_references)]
pub fn CloseSharedMem( h_map_file:MmapMut) {
    drop(h_map_file);
}


#[allow(non_snake_case)]
pub fn WriteSharedMemo(  mut hmapfile: MmapMut, pWritedata:*mut u8,wsize:u64) {
    unsafe{
        ptr::copy(pWritedata,hmapfile.as_mut_ptr(),wsize.try_into().unwrap());
}
}

#[allow(non_snake_case)]
pub fn ReadSharedMem( mut hmapfile: MmapMut, pReaddata:*mut u8, rdsize:u64) {
    unsafe{
        ptr::copy(hmapfile.as_mut_ptr(),pReaddata,rdsize.try_into().unwrap());
    }
} 

#[allow(non_snake_case)]
pub fn WriteAbleMem(memaddr:u64,esize:usize)->bool{
    unsafe {
        let mut oldprot=PAGE_PROTECTION_FLAGS(0);
        match  VirtualProtect(memaddr as *const c_void,esize,PAGE_EXECUTE_READWRITE,&mut oldprot) {
            Ok(_) => {
                true
            },
            Err(_e) => {
                debug_print!("write_able_mem error: 0x{:x},{:?}",memaddr,_e);
                false
            },
    }
}
}