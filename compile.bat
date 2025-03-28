@echo off
taskkill /F /IM cargo.exe /IM rustc.exe /IM Code.exe /IM explorer.exe /IM rust-analyzer.exe > nul 2>&1
rd /s /q target > nul 2>&1
start "" "C:\Program Files\Microsoft VS Code\Code.exe"
timeout /t 10 /nobreak > nul
set RUST_BACKTRACE=full
set SHUTTLE_ENV=local
cargo clean
cargo run