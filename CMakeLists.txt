cmake_minimum_required(VERSION 3.14)

if (WIN32)
    add_compile_options(/std:c++17)
    add_compile_options(/EHsc)
    add_compile_options(/permissive-)
else()
    # TODO: Linux and macOS
endif()

add_subdirectory(cpp-exe)
