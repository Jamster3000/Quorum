@echo off
setlocal enabledelayedexpansion
echo Running checks...

set /a total=5
set /a current=0

::FMT
set /a current+=1
echo.
echo [!current!/!total!] Formatting Rust code (src-tauri FMT)
cd src-tauri
call cargo fmt --all
if errorlevel 1 exit /b 1

::Clippy
set /a current+=1
echo [!current!/!total!] Linting Rust code (src-tauri Clippy)
call cargo clippy -- -W warnings
if errorlevel 1 exit /b 1

::Cargo check
set /a current+=1
echo [!current!/!total!] Cargo check...
call cargo check
if errorlevel 1 exit /b 1

::typescript checking
set /a current+=1
cd ../
echo [!current!/!total!] Type checking...
call npm run check
if errorlevel 1 exit /b 1

::run storybook's testing
set /a current+=1
echo [!current!/!total!] Storybook tests...
call npm run test:storybook
if errorlevel 1 exit /b 1

echo.
echo All checks passed!