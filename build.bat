@echo off
REM rustc main.rs
cls
cargo build
target\debug\TracingRaysInOneWeekend.exe > image.ppm && image.ppm
