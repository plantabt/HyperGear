use std::{ffi::{c_void, CStr, CString},   mem::size_of , ptr::{self,  null_mut}, sync::{Arc, Mutex}};

use tauri::Window;
use serde_json::{json, Value};
use windows::{core::{s, PCSTR, PCWSTR}, Win32::{Foundation::{ CloseHandle, BOOL, HANDLE,  INVALID_HANDLE_VALUE, MAX_PATH}, Security::{AdjustTokenPrivileges,  LookupPrivilegeValueW, LUID_AND_ATTRIBUTES, SE_DEBUG_NAME, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES}, System::{Diagnostics::{Debug::WriteProcessMemory, ToolHelp::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS}}, LibraryLoader::{GetModuleHandleA, GetProcAddress}, Memory::{VirtualAllocEx, MEM_COMMIT, PAGE_READWRITE}, Threading::{CreateRemoteThread, GetCurrentProcess, OpenProcess, OpenProcessToken, WaitForSingleObject, INFINITE, LPTHREAD_START_ROUTINE, PROCESS_ALL_ACCESS}}}};
use lazy_static::lazy_static;
#[derive(Clone, Default)]
#[allow(non_snake_case)]
pub struct AppInfo{
    pub app_window:Option<tauri::Window>
    
}
impl AppInfo{
      pub fn new()->Self{
        AppInfo::default()
    }

    pub fn call_ts_function(&self,event_name:String,pass_data:Value){
        if let Some(w) = &self.app_window{
            w.emit(event_name.as_str(), Some(pass_data)).expect("Failed to emit event");
        }
    }
    pub fn alert(&self,_title:String,_info:String){
        self.call_ts_function("front-event".to_owned(), json!({"event":"MessageBox","data":{"title":_title,"info":_info,"type":"alert"}}))
    }
}



lazy_static! {
    pub static ref G_APP_INFO: Arc<Mutex<Option<AppInfo>>> = Arc::new(Mutex::new(None));
}

pub fn init(window:Window){
    let mut app_info = G_APP_INFO.lock().unwrap();
    if let Some(app) = &mut *app_info{
        app.app_window = Some(window);
    }
}

#[allow(non_snake_case)]
pub fn GetProcessIdByName(process_name:String)->u32
{
    if process_name==""{
        return 0;
    }
    
    unsafe{
	let mut process_id = 0;
	let h_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();

	if h_snapshot !=INVALID_HANDLE_VALUE
	{
		let mut process_entry =PROCESSENTRY32 {dwSize: 0,
                                                        cntUsage: 0,
                                                        th32ProcessID: 0,
                                                        th32DefaultHeapID: 0,
                                                        th32ModuleID: 0,
                                                        cntThreads: 0,
                                                        th32ParentProcessID: 0,
                                                        pcPriClassBase: 0,
                                                        dwFlags: 0,
                                                        szExeFile: [0; 260], 
                                                    };
		process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
		if Process32First(h_snapshot, &mut process_entry as *mut PROCESSENTRY32).unwrap()==(){
			loop{
                let str_exe =CStr::from_ptr(process_entry.szExeFile.as_ptr()).to_string_lossy().into_owned();
				if str_exe==process_name {
					process_id = process_entry.th32ProcessID;
					break;
				}
                if Ok(()) != Process32Next(h_snapshot, &mut process_entry as *mut PROCESSENTRY32){
                    break;
                } 
			} 
		}
		CloseHandle(h_snapshot).unwrap();
	}
	process_id
}
}

pub fn alert(_title:String,_info:String){
    let  app_info = G_APP_INFO.lock().unwrap();
    if let Some(app) = & *app_info{
        app.alert(_title.to_owned(), _info.to_owned());
    }
}

pub fn make_md5_string(instr:String)->String{
    let mddig = md5::compute(instr);
    let md5char=[b'\x00';64];
    unsafe{
        ptr::copy::<u8>(format!("{:x}",mddig).as_ptr(),md5char.as_ptr() as *mut u8,32);
    }
    let btstr=String::from_utf8_lossy(&md5char).trim_end_matches('\u{0}').to_string();
    btstr
}

#[allow(non_snake_case)]
pub fn EnabledDebugPriilage()->bool{
    unsafe{
        let mut token=HANDLE(0);
        //提升权限
        match OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES, &mut token as *mut HANDLE){
            Ok(())=>{
                let luid = LUID_AND_ATTRIBUTES {
                    Luid: Default::default(),
                    Attributes: SE_PRIVILEGE_ENABLED,
                };
                let mut tkp=TOKEN_PRIVILEGES {
                    PrivilegeCount:1,
                    Privileges: [luid],
                };
                match LookupPrivilegeValueW(PCWSTR::null(), SE_DEBUG_NAME, &mut tkp.Privileges[0].Luid){
                    Ok(())=>{
                        tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
                        match AdjustTokenPrivileges(token, BOOL(0), Some(&tkp), size_of::<TOKEN_PRIVILEGES>() as u32, None, None){
                            Ok(())=>{
                                let _ = CloseHandle(token);
                                return true;
                            },
                            Err(_)=>{
                                alert( "[EnabledDebugPriilage] error".to_owned(), "AdjustTokenPrivileges failed!".to_owned());
                                return false;
                            }
                        }
                    },
                    Err(_)=>{
                        alert( "[EnabledDebugPriilage] error".to_owned(), "LookupPrivilegeValue failed!".to_owned());
                        return false;
                    }
                }
            }
            Err(_)=>{
                alert("[EnabledDebugPriilage] error".to_owned(), "OpenProcessToken failed!".to_owned());
                return false;
            }
        }
    }
} 

pub fn inject_dll(_pid:u32,_dll_path:String)->bool{
	let dw_buf_size = MAX_PATH as usize;
unsafe{
	//dwPID获取进程句柄
	if let Ok(h_process) = OpenProcess(PROCESS_ALL_ACCESS, false, _pid){
            let p_remote_buf = VirtualAllocEx(h_process, None, dw_buf_size, MEM_COMMIT, PAGE_READWRITE);
            if p_remote_buf!=null_mut(){
                let mut writebuf = [b'\x00';MAX_PATH as usize];
                ptr::copy(_dll_path.as_ptr() as *const u8,writebuf.as_mut_ptr(),_dll_path.len());
                if Ok(())== WriteProcessMemory(h_process, p_remote_buf, writebuf.as_ptr() as *const c_void , dw_buf_size, None){
                    if let Ok(h_module) = GetModuleHandleA(s!("kernel32.dll")){
                        if let Some(p_thread_proc) = GetProcAddress(h_module, s!("LoadLibraryA")){
                            if let Ok(h_thread) = CreateRemoteThread(h_process, None, 0,std::mem::transmute::<_, LPTHREAD_START_ROUTINE>(p_thread_proc),Some(p_remote_buf),0, None){
                                WaitForSingleObject(h_thread, INFINITE);
                                CloseHandle(h_thread).unwrap();
                                CloseHandle(h_process).unwrap();
                                return true;
                            }
                        }
                    }
                }
            }
        }else{
            alert("[inject_dll] error".to_owned(),"InjectDLL OpenProcess failed!".to_owned());
            return false; 
        }
    }
	false
}

pub fn print_hex(instr:String){
    for char in instr.chars() {
        print!("{:x} ", char as u8);
    }
}


#[allow(non_snake_case)]
pub fn StringToCStyle(instr:&String)->String{
    let mut tstr = instr.clone();
    tstr.push_str("\u{0}");
    tstr
}
#[allow(non_snake_case)]
pub fn StringToPCSTR(instr:&String)->CString{
     CString::new(instr.as_str()).expect("CString::new failed")
  
}
