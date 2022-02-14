# Install script for directory: /target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/raylib/src

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "TRUE")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/build/raylib/libraylib.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/raylib/src/raylib.h"
    "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/raylib/src/rlgl.h"
    "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/raylib/src/physac.h"
    "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/raylib/src/raymath.h"
    "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/raylib/src/raudio.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/build/raylib/raylib.pc")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/raylib" TYPE FILE FILES "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/build/raylib/raylib-config-version.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/raylib" TYPE FILE FILES "/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/raylib/src/../cmake/raylib-config.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/target/x86_64-pc-windows-gnu/release/build/raylib-sys-45223963e2ea3f85/out/build/raylib/external/glfw/cmake_install.cmake")

endif()

