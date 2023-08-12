@echo off
set RWA_WIN_ARCH=win-x64

echo Compilation for nw
set RWA_NODE_NW_VERSION=v0.49.2
cmd /C build nw --release

echo Compilation for nodejs
set RWA_NODE_NW_VERSION=v16.4.0
cmd /C build nodejs --release
