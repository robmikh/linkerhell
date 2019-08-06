cmake_minimum_required(VERSION 3.14)

if(UNIX AND NOT APPLE)
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
    add_compile_options(-stdlib=libc++)
    add_compile_options(-pthread)
    add_compile_options(-fexceptions)
    add_compile_options(-fPIC)
    add_compile_options(-g -gdwarf-4)

    if(LINUX)
       SET(CMAKE_SHARED_LINKER_FLAGS  "${CMAKE_SHARED_LINKER_FLAGS} -Wl,--no-undefined") 
       SET(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -lX11")
    endif()

    SET(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -stdlib=libc++")
endif()

add_definitions(
    -DLINUX=${LINUX}
    -DAPPLE=${APPLE}
)

add_subdirectory(cpp-exe)
