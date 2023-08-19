@echo off

chcp 65001
FOR /F "tokens=* USEBACKQ" %%g IN (`git rev-parse --abbrev-ref HEAD`) do (SET "GIT_BRANCH=%%g")
FOR /F "tokens=* USEBACKQ" %%g IN (`git describe --tags`) do (SET "GIT_TAG=%%g")
FOR /F "tokens=* USEBACKQ" %%g IN (`git rev-parse HEAD`) do (SET "GIT_LATEST_COMMIT_ID=%%g")

set RWA_WIN_ARCH=win-x64
cmd /C build nw --release
set RWA_WIN_ARCH=win-ia32
cmd /C build nw --release

set RWA_WIN_ARCH=win-x64
cmd /C build nodejs --release
set RWA_WIN_ARCH=win-ia32
cmd /C build nodejs --release