# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.22

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/raylib

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build

# Include any dependencies generated for this target.
include raylib/external/glfw/src/CMakeFiles/glfw.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include raylib/external/glfw/src/CMakeFiles/glfw.dir/compiler_depend.make

# Include the progress variables for this target.
include raylib/external/glfw/src/CMakeFiles/glfw.dir/progress.make

# Include the compile flags for this target's objects.
include raylib/external/glfw/src/CMakeFiles/glfw.dir/flags.make

# Object files for target glfw
glfw_OBJECTS =

# External object files for target glfw
glfw_EXTERNAL_OBJECTS = \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/context.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/init.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/input.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/monitor.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/vulkan.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/window.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_init.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_monitor.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_window.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/xkb_unicode.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/posix_time.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/posix_thread.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/glx_context.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/egl_context.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/osmesa_context.c.o" \
"/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/linux_joystick.c.o"

raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/context.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/init.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/input.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/monitor.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/vulkan.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/window.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_init.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_monitor.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_window.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/xkb_unicode.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/posix_time.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/posix_thread.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/glx_context.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/egl_context.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/osmesa_context.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw_objlib.dir/linux_joystick.c.o
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw.dir/build.make
raylib/external/glfw/src/libglfw3.a: raylib/external/glfw/src/CMakeFiles/glfw.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Linking C static library libglfw3.a"
	cd /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src && $(CMAKE_COMMAND) -P CMakeFiles/glfw.dir/cmake_clean_target.cmake
	cd /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/glfw.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
raylib/external/glfw/src/CMakeFiles/glfw.dir/build: raylib/external/glfw/src/libglfw3.a
.PHONY : raylib/external/glfw/src/CMakeFiles/glfw.dir/build

raylib/external/glfw/src/CMakeFiles/glfw.dir/clean:
	cd /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src && $(CMAKE_COMMAND) -P CMakeFiles/glfw.dir/cmake_clean.cmake
.PHONY : raylib/external/glfw/src/CMakeFiles/glfw.dir/clean

raylib/external/glfw/src/CMakeFiles/glfw.dir/depend:
	cd /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/raylib /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/raylib/src/external/glfw/src /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src /home/tcr-g/Rust/Nids2/target/debug/build/raylib-sys-19124d8b78513e2d/out/build/raylib/external/glfw/src/CMakeFiles/glfw.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : raylib/external/glfw/src/CMakeFiles/glfw.dir/depend

