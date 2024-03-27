// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::{fs::OpenOptions, mem::{size_of, transmute}};

use common_crate::{debug_print, mem::{CreateSharedMem, OpenSharedMem, ReadSharedMem, WriteSharedMemo}, sys::GetCurrentDir, utilis::{self, make_md5_string, AppInfo}};

use tauri::{  AppHandle, Manager, Window};
use utilis::{ EnabledDebugPriilage, inject_dll, GetProcessIdByName, G_APP_INFO};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod global;

#[tauri::command]
fn show(window:Window,visible:bool)->bool{
    
    if visible {
        window.show().expect("");
    }else{
        window.hide().expect("");
    } 
    true
}

#[tauri::command]
fn exit(app:AppHandle)->bool{
    app.exit(0);
    true
}
#[tauri::command]
fn minimize(window:Window)->bool{
    window.minimize().expect("");
    true
}
#[tauri::command]
fn drag_window(window: Window) {
    window.start_dragging().expect("Failed to set window draggable");
}
#[tauri::command]
fn message_box(title:String,info:String,msgtype:String){
    common_crate::utilis::alert(title,info);
}
#[tauri::command]
fn inject(window: Window,process_name:String,dllpath:String)->u32 {
    let pid = GetProcessIdByName(process_name.clone());
    debug_print!("{}",dllpath);
    debug_print!("{}",process_name);
    inject_dll(pid,dllpath.to_owned());

    return pid
}
#[tauri::command]
fn read_shared_mem(memname:String)->f32{
    let mut fspeed:f32 = 0.0;
    unsafe{

        match OpenSharedMem(memname){
            Ok(hmap)=>{
                
                ReadSharedMem(hmap,transmute(&mut fspeed),size_of::<f32>() as u64);
 
                debug_print!("ReadSharedMemo [o] : {}",fspeed);
            },
            Err(_)=>{
                debug_print!("ReadSharedMemo [x] : {}",fspeed);
            }
        }
       
    }
    fspeed
}

#[tauri::command]
fn write_shared_mem(memname:String,speed:f32){
    unsafe{

        match OpenSharedMem(memname){
            Ok(hmap)=>{
                let mut fspeed = speed;
                debug_print!("WriteSharedMemo [o] : {}",speed);
                WriteSharedMemo(hmap,transmute(&mut fspeed),size_of::<f32>() as u64);
            },
            Err(_)=>{
                debug_print!("WriteSharedMemo [x] : {}",speed);
            }
        }
       
    }

}

#[tauri::command]
fn create_shared_mem(memname:String,memsize:u64)->bool{

    match CreateSharedMem(memname.to_owned(), memsize){
        Ok(_)=>{
            true
        },
        Err(_)=>{
            false
        }
    }
}
#[tauri::command]
fn open_shared_mem(memname:String)->bool{
    match OpenSharedMem(memname.to_owned()){
        Ok(_)=>{
            true
        },
        Err(_)=>{
            false
        }
    }
}

#[tauri::command]
fn is_shared_mem(memname:String)->bool{
    
    let md5memname = make_md5_string(memname.to_owned());

    match OpenOptions::new()
        .read(true)
        .write(true)
        .open(md5memname.to_owned()){
            Ok(_)=>{
                debug_print!("IsSharedMem [o] : {}",md5memname);
                true
            },
            Err(_)=>{
                debug_print!("IsSharedMem [x] : {}",md5memname);
                false
            }
        }
}



#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
#[tauri::command]
fn get_appname() -> String {
    env!("CARGO_PKG_NAME").to_string()
}
#[tauri::command]
fn get_current_path()->String{
    GetCurrentDir()
}


fn main() {

    EnabledDebugPriilage();

    *G_APP_INFO.lock().unwrap() = Some(AppInfo::new());
    tauri::Builder::default()
    .setup(|app| {
        let window = app.get_window(get_appname().as_str()).unwrap();
        #[cfg(debug_assertions)] 
        {
            window.open_devtools();
        }
        utilis::init(window);
        Ok(())
    })
        .invoke_handler(tauri::generate_handler![show,
                                                exit,
                                                minimize,
                                                drag_window,
                                                inject,
                                                write_shared_mem,
                                                read_shared_mem,
                                                get_version,
                                                get_appname,
                                                message_box,
                                                create_shared_mem,
                                                is_shared_mem,
                                                open_shared_mem,
                                                get_current_path
                                                ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

