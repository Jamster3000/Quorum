@echo off
echo Running checks...

::FMT
echo.
echo [1/4] Formatting Rust code (src-tauri FMT)
call cd src-tauri
call cargo fmt --all
if errorlevel 1 exit /b 1

::Clippy
echo [2/4] Linting Rust code (src-tauri Clippy)
call cargo clippy -- -W warnings
if errorlevel 1 exit /b 1

::typescript checking
echo [3/4] Type checking...
cd ../
call npm run check
if errorlevel 1 exit /b 1

::run storybook's testing
echo [4/4] Storybook tests...
call npm run test:storybook
if errorlevel 1 exit /b 1

echo.
echo All checks passed!