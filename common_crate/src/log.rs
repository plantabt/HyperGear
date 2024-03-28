#[macro_export]
macro_rules! debug_print{
    ($($arg:tt)*)=>{
        #[cfg(debug_assertions)]{
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! create_log_window{
    ()=>{
       unsafe {
        #[cfg(debug_assertions)]
         AllocConsole(); 
        }
}
}
