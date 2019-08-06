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
    target_link_libraries(cpp-exe PRIVATE dl pthread)
    
    if (LINUX)
        target_link_libraries(cpp-exe PRIVATE rt)

        target_link_libraries(cpp-exe PRIVATE xcb X11)
    endif()

    if (APPLE)
        target_link_libraries(cpp-exe PRIVATE "-framework Security")
        target_link_libraries(cpp-exe PRIVATE "-framework CoreGraphics")
        target_link_libraries(cpp-exe PRIVATE "-framework Metal")
        target_link_libraries(cpp-exe PRIVATE "-framework IOKit")
        target_link_libraries(cpp-exe PRIVATE "-framework IOSurface")
        target_link_libraries(cpp-exe PRIVATE "-framework QuartzCore")
        target_link_libraries(cpp-exe PRIVATE "-framework CoreFoundation")
        target_link_libraries(cpp-exe PRIVATE "-framework Cocoa")
    endif()
endif()

set(LIB_NAME ${CMAKE_STATIC_LIBRARY_PREFIX}rust_lib${CMAKE_STATIC_LIBRARY_SUFFIX})
string(TOLOWER ${CMAKE_BUILD_TYPE} CARGO_BUILD_TYPE)
target_link_libraries(cpp-exe PUBLIC ${CMAKE_BINARY_DIR}/rust-lib/${CARGO_BUILD_TYPE}/${LIB_NAME})

install (TARGETS cpp-exe DESTINATION output)