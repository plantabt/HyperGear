use std::{ fs, path::Path};

fn main() {
    let red = "\x1b[31m";
   // let yellow = "\x1b[33m";
    let reset = "\x1b[0m"; 

    let mut source = Path::new("../../target/release/gear.dll");
    let mut errinfo ="error: Please run [ cargo build -p gear --release ] first!!!";
    #[cfg(debug_assertions)]{
        source = Path::new("../../target/debug/gear.dll");
        errinfo="error: Please run [ cargo build -p gear ] first!!!";
    }
    let destination = Path::new("./gear.dll");
    println!("copy gear.dll -->{:?} - {:?}",source,destination);
    match fs::copy(&source, &destination) {
        Ok(_) => println!("File copied successfully to {:?}", destination),

        Err(_) => {
            print!("{}", red);
            print!("{}", errinfo);
            println!("{}", reset);
        },
    }
    tauri_build::build()
}
