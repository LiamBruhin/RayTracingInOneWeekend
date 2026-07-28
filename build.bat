@echo off
REM rustc main.rs
cargo build
target\debug\TracingRaysInOneWeekend.exe > image.ppm && image.ppm
