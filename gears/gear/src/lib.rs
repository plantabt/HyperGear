mod hook;
mod hookimpl;
mod utlits;
mod global;
use common_crate::{create_log_window, sys::GetCurrentExeName};
use global::G_EXE_NAME;
use hookimpl::{initGetTickCount, initGetTickCount64, initRtlQueryPerformanceCounter, init_timeGetTime};
use once_cell::sync::Lazy;
use winapi::um::consoleapi::AllocConsole;
use windows::Win32::Foundation::{BOOLEAN, HMODULE};


const   DLL_PROCESS_ATTACH:u32  =    1;
const   DLL_THREAD_ATTACH:u32   =    2;
const   DLL_THREAD_DETACH:u32   =    3;
const   DLL_PROCESS_DETACH:u32  =    0;




#[no_mangle]
extern "system" fn DllMain(_hmodule:HMODULE,ul_reason_for_call:u32,_lp_reserved:& u32)->BOOLEAN{

    unsafe {
        match ul_reason_for_call{
            DLL_PROCESS_ATTACH=>{
                create_log_window!();

                G_EXE_NAME = Lazy::new(|| String::from(GetCurrentExeName()));
 
                init_timeGetTime();
                initGetTickCount64();
                initGetTickCount();
                initRtlQueryPerformanceCounter();
                 
            },
            DLL_THREAD_ATTACH=>{
                
            },
            DLL_THREAD_DETACH=>{

            },
            DLL_PROCESS_DETACH=>{

            },
            _ =>{}
        };
        BOOLEAN(1)
    }
}