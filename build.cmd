@echo off
@REM %~1 = nodejs | nw | run | clean
@REM RWA_WIN_ARCH = win-ia32 | win-x64 | win-arm64
if "%RWA_WIN_ARCH%" == "win-ia32" (
    set RUSTUP_TOOLCHAIN=nightly-i686-pc-windows-msvc
    set CARGO_TARGET_DIR=target-%RWA_WIN_ARCH%
)
setlocal
set RWA_PKG_NAME=request-window-attention
set RWA_DIST_DIR=".\dist\%~1\%RWA_WIN_ARCH%"
if "%~1"=="run" (
    cargo run --package "%~3" --bin "%~5"
) else if "%~1"=="clean" (
    cargo clean
    set CARGO_TARGET_DIR=target-win-ia32
    cargo clean
    if exist .\dist (
        rmdir /s /q .\dist
    )
) else (
    if not exist %RWA_DIST_DIR% (
        mkdir %RWA_DIST_DIR%
    )
    echo ***********************************************
    echo *** Compilation for %~1 - %RWA_WIN_ARCH% ****
    echo ***********************************************
    nj-cli build --out=%RWA_DIST_DIR% %~2 -- --features=%~1
    move /y %RWA_DIST_DIR%\index.node %RWA_DIST_DIR%\%RWA_PKG_NAME%.node
    copy .\%RWA_PKG_NAME%.d.ts %RWA_DIST_DIR%\%RWA_PKG_NAME%.d.ts
)
endlocal
