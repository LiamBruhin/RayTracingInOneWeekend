@echo off
rustc main.rs
main.exe > image.ppm && image.ppm
