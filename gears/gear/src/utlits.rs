use std::mem::{size_of, transmute};
use common_crate::{debug_print, mem::{OpenSharedMem, ReadSharedMem}};
use crate::global::G_EXE_NAME;

#[allow(non_snake_case)]
pub fn ReadSpeed()->f32{
    unsafe{
    match OpenSharedMem(G_EXE_NAME.to_owned()){
        Ok(hmapping)=>{
            let mut fspeed:f32=0.0;
            ReadSharedMem(hmapping, transmute(&mut fspeed), size_of::<f32>() as u64);
            fspeed
        }
        Err(e) =>{
            debug_print!("dll OpenSharedMem failed: {:?}",e);
            1.0
        },
    }
}
}

