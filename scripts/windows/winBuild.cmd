@echo off
setlocal

pushd %~dp0..\..\
set "REPO_ROOT_PATH=%CD%"
popd

set BUILDPATH=%REPO_ROOT_PATH%\_build\windows

pushd %REPO_ROOT_PATH%\rust-lib
set CARGO_TARGET_DIR=%BUILDPATH%\rust-lib
cargo build
popd

cmake . "-B%BUILDPATH%" -GNinja -DCMAKE_BUILD_TYPE=Debug "-DCMAKE_INSTALL_PREFIX=%BUILDPATH%" 
ninja -C "%BUILDPATH%" install
