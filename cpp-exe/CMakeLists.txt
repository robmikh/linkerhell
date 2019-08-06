project(cpp-exe)

set(sources main.cpp)

include_directories(${CMAKE_BINARY_DIR}/rust-lib/bindings)

add_executable(cpp-exe ${sources})

if (WIN32)
    target_link_libraries(cpp-exe PRIVATE WindowsApp.lib)

    # winit introduces these dependencies
    target_link_libraries(cpp-exe PRIVATE dwmapi.lib userenv.lib)

    # wgpu introduces these dependencies
    target_link_libraries(cpp-exe PRIVATE d3d12.lib d3dcompiler.lib dxgi.lib)
else()
    # TODO: Linux and macOS
endif()

string(TOLOWER ${CMAKE_BUILD_TYPE} CARGO_BUILD_TYPE)
target_link_libraries(cpp-exe PUBLIC ${CMAKE_BINARY_DIR}/rust-lib/${CARGO_BUILD_TYPE}/rust_lib.lib)

install (TARGETS cpp-exe DESTINATION output)