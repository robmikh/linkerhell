@echo off
setlocal

set CMAKE_BUILD_TYPE=Debug
set CARGO_BUILD_FLAG=
if "%1"=="release" (
    set CMAKE_BUILD_TYPE=Release
    set CARGO_BUILD_FLAG="--release"
) 

pushd %~dp0..\..\
set "REPO_ROOT_PATH=%CD%"
popd

set BUILDPATH=%REPO_ROOT_PATH%\_build\windows

pushd %REPO_ROOT_PATH%\rust-lib
set CARGO_TARGET_DIR=%BUILDPATH%\rust-lib
cargo build --features dx12 %CARGO_BUILD_FLAG%
popd

cmake . "-B%BUILDPATH%" -GNinja "-DCMAKE_BUILD_TYPE=%CMAKE_BUILD_TYPE%" "-DCMAKE_INSTALL_PREFIX=%BUILDPATH%" 
ninja -C "%BUILDPATH%" install
