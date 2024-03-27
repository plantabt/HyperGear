use std::{mem::transmute, ops::{Add, Mul, Sub}};

use common_crate::{debug_print, sys::GetFuncAddress};
use num_traits::{NumCast, ToPrimitive};
use winapi::{shared::ntdef::PLARGE_INTEGER, um::winnt::LARGE_INTEGER};

use windows::Win32::Foundation::NTSTATUS;

use crate::{global::{GET_TICK_COUNT, GET_TICK_COUNT64, RTL_QUERY_PERFORMANCE_COUNTER_HACKER, TIME_GET_TIME}, hook::HOOK, utlits::ReadSpeed};

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Clone, Default,Copy)]
pub struct SpeedHackers <T>where
T: Sub<Output = T> + Mul<T, Output = T> + Add<Output = T> + Copy + NumCast + ToPrimitive,
{
    pub hook_impl: HOOK,
    pub speed:f32,
    pub initialtime:T,
    pub initialoffset:T,
}

impl  <T>SpeedHackers<T> where
T: Sub<Output = T> + Mul<T, Output = T> + Add<Output = T> + Copy+ NumCast + ToPrimitive,
{
    pub fn new(speed:f32, initialtime: T, initialoffset: T)->Self{
        Self { hook_impl: HOOK::new(), speed, initialtime, initialoffset }
      
    }

    pub fn get(self,current_time:T)->T {  
        let time_delta = current_time - self.initialtime;
        let result = (time_delta.to_f32().unwrap() * self.speed)+ self.initialoffset.to_f32().unwrap();
        NumCast::from(result).unwrap()
   }
}
pub type RtlQueryPerformanceCounterType=fn(PerformanceCounter:PLARGE_INTEGER)->NTSTATUS;
pub type GetTickCountType=fn()->u32;
pub type GetTickCount64Type=fn()->i64;
pub type TimeGetTimeType=fn()->u32;

#[no_mangle]
#[allow(non_snake_case)]
extern "C" fn fakeRtlQueryPerformanceCounter(pPerformanceCounter:PLARGE_INTEGER)->NTSTATUS{
unsafe{
        let mut result:NTSTATUS=NTSTATUS(0);
        //original function excuted first.
        RTL_QUERY_PERFORMANCE_COUNTER_HACKER.speed = ReadSpeed();
        debug_print!("speed is: {}",RTL_QUERY_PERFORMANCE_COUNTER_HACKER.speed);
        let addr = RTL_QUERY_PERFORMANCE_COUNTER_HACKER.hook_impl.bridge_addr;
        if addr==0 {
            return result;
        }
       
        result = transmute::<u64,RtlQueryPerformanceCounterType>(addr)(pPerformanceCounter) ;
 
        debug_print!("inRtlQueryPerformanceCounter: begin {}",*(*pPerformanceCounter).QuadPart());
        let newtime = RTL_QUERY_PERFORMANCE_COUNTER_HACKER.get(*(*pPerformanceCounter).QuadPart());
        let  pper =pPerformanceCounter;
        (*pper).u_mut().HighPart = (newtime>>32) as i32;
        (*pper).u_mut().LowPart = (newtime&0x00000000FFFFFFFF) as u32;        

        debug_print!("inRtlQueryPerformanceCounter: end {} ",*(*pPerformanceCounter).QuadPart());
        result
}

}

#[no_mangle]
#[allow(non_snake_case)]
extern "C" fn fakeGetTickCount()->u32{
unsafe{
        let mut result:u32=0;
        //original function excuted first.
        GET_TICK_COUNT.speed = ReadSpeed();
        debug_print!("fakeGetTickCount speed is: {}",GET_TICK_COUNT.speed);
        let addr = GET_TICK_COUNT.hook_impl.bridge_addr;
        if addr==0 {
            return result;
        }
       
        result = transmute::<u64,GetTickCountType>(addr)() ;

        debug_print!("fakeGetTickCount: begin {}",result);
        let newtime = GET_TICK_COUNT.get(result as i32);

        debug_print!("fakeGetTickCount: end {} ",newtime as u32);
        newtime as u32
}

}

#[no_mangle]
#[allow(non_snake_case)]
extern "C" fn fakeGetTickCount64()->i64{
unsafe{
        let mut result:i64=0;
        //original function excuted first.
        GET_TICK_COUNT64.speed = ReadSpeed();
        debug_print!("fakeGetTickCount64 speed is: {}",GET_TICK_COUNT64.speed);
        let addr = GET_TICK_COUNT64.hook_impl.bridge_addr;
        if addr==0 {
            return result;
        }
       
        result = transmute::<u64,GetTickCount64Type>(addr)() ;

        debug_print!("fakeGetTickCount64: begin {}",result);
        let newtime = GET_TICK_COUNT64.get(result as i64);

        debug_print!("fakeGetTickCount64: end {} ",newtime as u64);
        newtime as i64
}

}

#[no_mangle]
#[allow(non_snake_case)]
extern "C" fn fake_timeGetTime()->u32{
unsafe{
        let mut result:u32=0;
        //original function excuted first.
        TIME_GET_TIME.speed = ReadSpeed();
        debug_print!("fake_timeGetTime speed is: {}",TIME_GET_TIME.speed);
        let addr = TIME_GET_TIME.hook_impl.bridge_addr;
        if addr==0 {
            return result;
        }
       
        result = transmute::<u64,TimeGetTimeType>(addr)() ;

        debug_print!("fake_timeGetTime: begin {}",result);
        let newtime = TIME_GET_TIME.get(result as i32);

        debug_print!("fake_timeGetTime: end {} ",newtime);
        newtime as u32
}

}
//////////////////////////////// init ///////////////////////////////////////////////

#[allow(non_snake_case)]
pub fn init_timeGetTime(){
    unsafe{
        const HOOK_FUNC_HANE: &str ="timeGetTime";
        const HOOK_DLL_NAME: &str = "winmm.dll";
        const FAKE_FUNC: extern "C" fn() -> u32 = fake_timeGetTime;
        
        let original_addr = GetFuncAddress(HOOK_DLL_NAME,HOOK_FUNC_HANE);
        debug_print!("{}: {:?}",HOOK_DLL_NAME,original_addr);
   
 
       TIME_GET_TIME.hook_impl.set_hook_iat(HOOK_DLL_NAME,HOOK_FUNC_HANE,FAKE_FUNC as u64);

       TIME_GET_TIME.hook_impl.bridge_addr = original_addr;
        let addr  = TIME_GET_TIME.hook_impl.bridge_addr;
        if addr==0{
            return;
        }
        TIME_GET_TIME.initialtime=transmute::<u64,TimeGetTimeType>(original_addr)() as i32;
        TIME_GET_TIME.initialoffset=transmute::<u64,TimeGetTimeType>(addr)() as i32;
        debug_print!("init_timeGetTime: {:?} - {:?}",TIME_GET_TIME.initialtime,TIME_GET_TIME.initialoffset); 
    }
}

#[allow(non_snake_case)]
pub fn initGetTickCount64(){
    unsafe{
        const HOOK_FUNC_HANE: &str ="GetTickCount64";
        const HOOK_DLL_NAME: &str = "kernel32.dll";
        const FAKE_FUNC: extern "C" fn() -> i64 = fakeGetTickCount64;
      
        
        let original_addr = GetFuncAddress(HOOK_DLL_NAME,HOOK_FUNC_HANE);
        debug_print!("{}: {:?}",HOOK_DLL_NAME,original_addr);
   
       GET_TICK_COUNT64.hook_impl.set_hook(original_addr as i64,FAKE_FUNC as i64);


        let addr  = GET_TICK_COUNT64.hook_impl.bridge_addr;
        if addr==0{
            return;
        }
        GET_TICK_COUNT64.initialtime=transmute::<u64,TimeGetTimeType>(original_addr)() as i64;
        GET_TICK_COUNT64.initialoffset=transmute::<u64,TimeGetTimeType>(addr)() as i64;
        debug_print!("initGetTickCount64: {:?} - {:?}",GET_TICK_COUNT64.initialtime,GET_TICK_COUNT64.initialoffset); 
    }
}

#[allow(non_snake_case)]
pub fn initGetTickCount(){
    unsafe{
        const HOOK_FUNC_HANE: &str ="GetTickCount";
        const HOOK_DLL_NAME: &str = "kernel32.dll";
        const FAKE_FUNC: extern "C" fn() -> u32 = fakeGetTickCount;
        
        let original_addr = GetFuncAddress(HOOK_DLL_NAME,HOOK_FUNC_HANE);
        debug_print!("{}: {:?}",HOOK_DLL_NAME,original_addr);
   
        GET_TICK_COUNT.hook_impl.set_hook(original_addr as i64,FAKE_FUNC as i64);


        let addr  = GET_TICK_COUNT.hook_impl.bridge_addr;
        if addr==0{
            return;
        }
        GET_TICK_COUNT.initialtime=transmute::<u64,GetTickCountType>(original_addr)() as i32;
        GET_TICK_COUNT.initialoffset=transmute::<u64,GetTickCountType>(addr)() as i32;
        debug_print!("initGetTickCount: {:?} - {:?}",GET_TICK_COUNT.initialtime,GET_TICK_COUNT.initialoffset); 
    }
}
#[allow(non_snake_case)]
pub fn initRtlQueryPerformanceCounter(){
    unsafe{
        const HOOK_FUNC_HANE: &str ="RtlQueryPerformanceCounter";
        const HOOK_DLL_NAME: &str = "ntdll.dll";
        const FAKE_FUNC: extern "C" fn(*mut LARGE_INTEGER) -> NTSTATUS= fakeRtlQueryPerformanceCounter;
        
        let original_addr = GetFuncAddress(HOOK_DLL_NAME,HOOK_FUNC_HANE);

        debug_print!("{}: {:?}",HOOK_FUNC_HANE,original_addr);
        
        debug_print!("original_addr: {:?}, fakeRtlQueryPerformanceCounter: 0x{:x}",original_addr,FAKE_FUNC as u64);
        RTL_QUERY_PERFORMANCE_COUNTER_HACKER.hook_impl.set_hook(original_addr as i64,FAKE_FUNC as i64);

        let mut inittime:u64=0;
        let mut initialoffset:u64=0;
        
        let addr  = RTL_QUERY_PERFORMANCE_COUNTER_HACKER.hook_impl.bridge_addr;
        if addr==0{
            return;
        }
        transmute::<u64,RtlQueryPerformanceCounterType>(original_addr)(&mut inittime as *mut _ as PLARGE_INTEGER);
        transmute::<u64,RtlQueryPerformanceCounterType>(addr)(&mut initialoffset as *mut _ as PLARGE_INTEGER);

        RTL_QUERY_PERFORMANCE_COUNTER_HACKER.initialtime=inittime as i64;
        RTL_QUERY_PERFORMANCE_COUNTER_HACKER.initialoffset=initialoffset as i64;
        debug_print!("RtlQueryPerformanceCounter: {:?} - {:?}",inittime,initialoffset);
    }
}
