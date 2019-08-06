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
else()
    add_compile_options(-std=c++17)
    add_compile_options(-pthread)

    if (APPLE)
        set(CMAKE_SHARED_LINKER_FLAGS ${CMAKE_SHARED_LINKER_FLAGS} -framework CoreGraphics -framework CoreFoundation -framework Metal)
    endif()
endif()

add_subdirectory(cpp-exe)
