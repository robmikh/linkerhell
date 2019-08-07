cmake_minimum_required(VERSION 3.5)

if (UNIX AND NOT APPLE)
    set(LINUX 1)
else()
    set(LINUX 0)
endif()

if (NOT APPLE)
    set(APPLE 0)
endif()

if (WIN32)
    add_compile_options(/std:c++17)
    add_compile_options(/EHsc)
    add_compile_options(/permissive-)
    
    # This will resolve the linker issue when building in Debug mode...
    # ... but it's not that different from building in Release mode.
    #add_compile_options(/MD)
else()
    add_compile_options(-std=c++17)
    add_compile_options(-pthread)
endif()

add_subdirectory(cpp-exe)
