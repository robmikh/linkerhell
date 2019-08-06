project(cpp-exe)

set(sources main.cpp)

include_directories(${CMAKE_BINARY_DIR}/rust-lib/bindings)

add_executable(cpp-exe ${sources})

if (WIN32)
    target_link_libraries(cpp-exe PRIVATE WindowsApp.lib)

    # winit introduces these dependencies
    target_link_libraries(cpp-exe PRIVATE dwmapi.lib userenv.lib)
else()
    # TODO: Linux and macOS
endif()

target_link_libraries(cpp-exe PUBLIC ${CMAKE_BINARY_DIR}/rust-lib/debug/rust_lib.lib)

install (TARGETS cpp-exe DESTINATION output)