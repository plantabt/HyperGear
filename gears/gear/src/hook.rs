
use core::slice;
use std::{arch::global_asm, mem::{size_of, transmute}, ptr};

use common_crate::{debug_print, mem::WriteAbleMem, sys::{GetCurrentExeName, GetFuncAddress}, utilis::StringToPCSTR};
use num_traits::ToPrimitive;
use winapi::um::winnt::{IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_IMPORT_DESCRIPTOR, IMAGE_THUNK_DATA, PIMAGE_DOS_HEADER, PIMAGE_IMPORT_DESCRIPTOR, PIMAGE_NT_HEADERS, PIMAGE_THUNK_DATA};
use windows::{core::PCSTR, Win32::{Foundation::HWND, System::{LibraryLoader::GetModuleHandleA, Memory::{VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE}}, UI::WindowsAndMessaging::MessageBoxA}};
use iced_x86::{Decoder, DecoderOptions, Instruction};

global_asm!("
.global inline_entry_point
inline_entry_point:
Mov Rax, Rsp
And Spl, 0xF0
Sub Rsp, 0x20
Mov [Rsp + 8], Rax
Mov Rax, [Rax - 8]
Sub Rsp, 0x100
Mov Rax,0x1122334455667788
call Rax
Add Rsp, 0x100
Mov Rsp, [Rsp + 8]
Ret
");
extern "C" { fn inline_entry_point();}


#[allow(non_snake_case)]
pub fn GetImportTableFuncAddr(baseAddr:u64,dllname:&str,funcname:&str)->u64 {
    let togetaddr = GetFuncAddress(dllname,funcname);
    if togetaddr==0{
        return 0;
    }

    unsafe {
        let pDosHeader = baseAddr as PIMAGE_DOS_HEADER;
        let  pNTHeader = (((*pDosHeader).e_lfanew as u64) + (baseAddr as u64)) as PIMAGE_NT_HEADERS;
        let idd = &(*pNTHeader).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT as usize];
        let mut pID = ((idd.VirtualAddress as u64)+(baseAddr as u64)) as PIMAGE_IMPORT_DESCRIPTOR;
        //loop import table
        loop{
            if (*pID).u.Characteristics().to_i32() == Some(0) {
                break;
            }
            let mut pRealIAT = ((baseAddr as u64) + ((*pID).FirstThunk as u64 )) as PIMAGE_THUNK_DATA;
            for _i in 0..1024{
                let funiat = (*pRealIAT).u1.Function().to_u64().unwrap();
                if funiat==0{
                    break;
                }
                
                if funiat==togetaddr{
                    debug_print!("function({}): iat addr: 0x{:x}",_i,pRealIAT as u64);
                    return pRealIAT as u64;
                }
                
               
                pRealIAT = ((pRealIAT as u64) + (size_of::<IMAGE_THUNK_DATA>() as u64)) as PIMAGE_THUNK_DATA;
            }
            //increase dll import addr
            pID = ((pID as u64) + (size_of::<IMAGE_IMPORT_DESCRIPTOR>() as u64)) as PIMAGE_IMPORT_DESCRIPTOR;
        }
        //dll name
    }
    0
}

#[repr(C)]
#[derive(Clone, Default,Copy)]
pub struct HOOK{
    pub original_addr:i64,
    pub newcall_addr:i64,
    pub bridge_addr:u64,
    pub bridge_code_size:i32,
    pub inline_size:i32,
    pub orig_codesize:i32,
}

impl HOOK {

    pub  fn new()->Self{
        let inline_size = 15;
        //create bridge
        let mut result = HOOK::default();
        result.inline_size=inline_size;
        result

    }

    pub fn set_hook_iat(&mut self,dllname:&str,funcname:&str,newfunc:u64){
        unsafe{
            let exename = GetCurrentExeName();
            let t_exename = PCSTR(StringToPCSTR(&exename.to_string()).into_raw() as *mut u8);

            match GetModuleHandleA(t_exename){
                Ok(baseaddr) => {
                    let replace = GetImportTableFuncAddr(baseaddr.0 as u64,dllname,funcname);
                    
                    if WriteAbleMem(replace,0x10)==true{
                        ptr::write::<u64>(replace as *mut u64,newfunc);
                    }
                },
                Err(_) => {
                    debug_print!("set_hook_iat module not found : {}",exename);
                },
            }
        }
    }

    fn establish_inline_point(&self,new_func:u64)->u64 {
        unsafe{
            const INLINE_POINT_SIZE: i32 = 0x50;
            let t_new_func = VirtualAlloc( None, INLINE_POINT_SIZE as usize, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE) as *mut u8;
            ptr::copy(inline_entry_point as *mut u8,t_new_func,INLINE_POINT_SIZE as usize);
            ptr::write::<u64>(t_new_func.offset(0x1d) as *mut u64, transmute(new_func));
            t_new_func as u64
        }
    }
    pub fn set_hook(&mut self,original_func:i64,new_func:i64){
        unsafe {
            let t_new_func = self.establish_inline_point(new_func as u64);

            self.newcall_addr = t_new_func as i64;
            self.original_addr = original_func;
            
            self.orig_codesize = get_instruction_size(original_func as i64,self.inline_size as i64) as i32;
            self.bridge_code_size = self.orig_codesize+self.inline_size*2;

            self.bridge_addr = std::mem::transmute(VirtualAlloc( None, self.bridge_code_size.try_into().unwrap(), MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE));
  

            ptr::copy(self.original_addr as *mut u8,std::mem::transmute(self.bridge_addr),self.orig_codesize.try_into().unwrap());
            //write bridge
            let pcpy:*mut u8 = std::mem::transmute(self.bridge_addr);

            ptr::write::<u64>(pcpy.offset(self.orig_codesize as isize) as *mut u64, 0x0000000025FF48);
            ptr::write::<i64>(pcpy.offset(self.orig_codesize as isize +7) as *mut i64, self.original_addr+self.orig_codesize as i64);


            if WriteAbleMem(self.original_addr as u64,self.inline_size as usize)==true{
                let pbyte = self.original_addr  as *mut u8;
                // 48:FF25 A9B70600                           | jmp qword ptr ds:[<LoadLibraryA>]                                                      |
                //write jmp qword ptr[0]
                ptr::write::<u64>(pbyte as *mut u64, 0x0000000025FF48); 
                //write newFunc addr
                ptr::write::<i64>(pbyte.offset(7) as *mut i64, self.newcall_addr);
               //nop
               for i in 0..(self.orig_codesize-self.inline_size){
                   ptr::write(pbyte.offset(self.inline_size as isize +i as isize), 0x90);
               }
            }

            };
    }
    pub fn delete(&mut self){
        unsafe {
             VirtualFree(std::mem::transmute(self.bridge_addr), self.bridge_code_size.try_into().unwrap(), MEM_RELEASE).unwrap();
         };
        
    }

}


fn get_instruction_size(_addr:i64,dest_size:i64)->i64{
    let inline_size = 15;
        
    let mut orig_codesize:usize=0;
    let bytes = _addr as *const u8;
    let data_buff:&[u8]  =unsafe{
        slice::from_raw_parts(bytes,dest_size as usize +0x20)
    };
    let mut decoder =    Decoder::with_ip(64, data_buff, _addr as u64, DecoderOptions::NONE);
    let mut instruction = Instruction::default();

    while decoder.can_decode(){
        decoder.decode_out(&mut instruction);
        orig_codesize += instruction.len();

        if orig_codesize>=inline_size {
            break;
        }
    }
    orig_codesize as i64
}
