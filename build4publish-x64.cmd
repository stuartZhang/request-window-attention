@echo off

set RWA_WIN_ARCH=win-x64

set RWA_NODE_NW_VERSION=v0.49.2
echo Compilation for nw-%RWA_NODE_NW_VERSION%-%RWA_WIN_ARCH%
cmd /C build nw --release

set RWA_NODE_NW_VERSION=v0.54.1
echo Compilation for nw-%RWA_NODE_NW_VERSION%-%RWA_WIN_ARCH%
cmd /C build nw --release

set RWA_NODE_NW_VERSION=v0.72.0
echo Compilation for nw-%RWA_NODE_NW_VERSION%-%RWA_WIN_ARCH%
cmd /C build nw --release

set RWA_NODE_NW_VERSION=v16.4.0
echo Compilation for nodejs-%RWA_NODE_NW_VERSION%-%RWA_WIN_ARCH%
cmd /C build nodejs --release
