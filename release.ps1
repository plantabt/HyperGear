cargo build -p gear --release
Copy-Item "./target/release/gear.dll" "./HyperGear/src-tauri/gear.dll" -Force
cargo tauri build
Remove-Item "./HyperGear/src-tauri/gear.dll" -Force