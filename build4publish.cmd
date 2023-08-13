@echo off

set RWA_WIN_ARCH=win-x64

set RWA_NODE_NW_VERSION=v0.49.2
cmd /C build nw --release

set RWA_NODE_NW_VERSION=v0.54.1
cmd /C build nw --release

set RWA_NODE_NW_VERSION=v0.72.0
cmd /C build nw --release

set RWA_NODE_NW_VERSION=v16.4.0
cmd /C build nodejs --release
