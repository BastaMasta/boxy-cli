@echo off
REM Windows commands
echo Building Library...
cd .\boxy-cli
cargo build
echo Running Binary...
cd ..\binary-testing
cargo run