# linkerissuerepro
A project that reproduces a linking issue. The issue only seems to reproduce on Windows when doing a debug build. There is now an (imperfect) solution identified.

## The Project
This is a minimal repro of the issue taken from a larger project. In short, `rust-lib` is a rust component that builds into a static lib. This lib is then linked into `cpp-exe`, a C++ executable, which consumes it.

## The Issue
~~On Windows, `spirv-cross` seems to build for Release despite the rest of the project building for Debug. Not sure how that's happening ¯\\\_(ツ)_/¯~~ It appears that Rust always uses the release CRT, so this shouldn't be unique to `spirv-cross`.

The issue goes away when building the entire project for Release. Here's the error when building for Debug:
```
rust_lib.lib(wrapper.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(wrapper.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
rust_lib.lib(spirv_cross.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(spirv_cross.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
rust_lib.lib(spirv_cross_util.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(spirv_cross_util.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
rust_lib.lib(spirv_glsl.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(spirv_glsl.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
rust_lib.lib(spirv_hlsl.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(spirv_hlsl.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
rust_lib.lib(spirv_msl.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(spirv_msl.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
rust_lib.lib(spirv_cfg.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(spirv_cfg.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
rust_lib.lib(spirv_cross_parsed_ir.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(spirv_cross_parsed_ir.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
rust_lib.lib(spirv_parser.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2' in main.cpp.obj
rust_lib.lib(spirv_parser.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug' in main.cpp.obj
LINK : warning LNK4098: defaultlib 'MSVCRT' conflicts with use of other libs; use /NODEFAULTLIB:library
cpp-exe\cpp-exe.exe : fatal error LNK1319: 18 mismatches detected
ninja: build stopped: subcommand failed.
```

## Building
There are scripts to build for each platform. Currently Windows, Linux (tested with popOS), and macOS are supported. By default, the project builds in Debug mode. Pass "release" to the script to build in Release mode.

Make sure you have [Ninja](https://ninja-build.org/) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed. On Windows, this project builds with MSVC. All other platforms build with clang.

### Windows
```
cd linkerhell
.\scripts\windows\winBuild.cmd
```

### Linux
```
cd linkerhell
./scripts/linux/linuxBuild.sh
```

### macOS
```
cd linkerhell
./scripts/macOS/macOSBuild.sh
```

## Solution
It's an imperfect solution, but compiling with `/MD` resolved the issue on Windows.
