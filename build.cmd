@echo off
@REM RWA_WIN_ARCH = win-x86 | win-x64 | win-arm64
set RWA_NODE_VERSION=v16.4.0
set RWA_WIN_ARCH=win-x64
setlocal
set RWA_PKG_NAME=request-window-attention
if "%~1"=="run" (
    cargo run --package "%~3" --bin "%~5"
) else if "%~1"=="clean" (
    cargo clean
    if exist .\dist (
        rmdir /s /q .\dist
    )
) else (
    mkdir .\dist\%RWA_NODE_VERSION%\%RWA_WIN_ARCH%
    nj-cli build --out=.\dist\%RWA_NODE_VERSION%\%RWA_WIN_ARCH% %* -- --features=nodejs
    move /y .\dist\%RWA_NODE_VERSION%\%RWA_WIN_ARCH%\index.node .\dist\%RWA_NODE_VERSION%\%RWA_WIN_ARCH%\%RWA_PKG_NAME%.node
    copy .\%RWA_PKG_NAME%.d.ts .\dist\%RWA_NODE_VERSION%\%RWA_WIN_ARCH%\%RWA_PKG_NAME%.d.ts
)
endlocal
