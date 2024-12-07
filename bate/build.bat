@echo off
REM Build release binaries for macOS, Linux, and Windows

echo Installing necessary targets...
cargo install cross --force >nul 2>&1 || echo "cross already installed"

REM Set target names
set MACOS_TARGET=x86_64-apple-darwin
set LINUX_TARGET=x86_64-unknown-linux-gnu
set WINDOWS_TARGET=x86_64-pc-windows-gnu

echo Building for macOS...
cross build --release --target %MACOS_TARGET%
if %errorlevel% neq 0 (
    echo Failed to build for macOS!
    exit /b %errorlevel%
)

echo Building for Linux...
cross build --release --target %LINUX_TARGET%
if %errorlevel% neq 0 (
    echo Failed to build for Linux!
    exit /b %errorlevel%
)

echo Building for Windows...
cross build --release --target %WINDOWS_TARGET%
if %errorlevel% neq 0 (
    echo Failed to build for Windows!
    exit /b %errorlevel%
)

echo Build complete. Check the target/release directories for your binaries.
pause
