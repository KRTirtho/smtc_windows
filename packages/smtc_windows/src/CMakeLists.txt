# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(smtc_windows_library VERSION 0.0.1 LANGUAGES C)

add_library(smtc_windows SHARED
  "smtc_windows.c"
)

set_target_properties(smtc_windows PROPERTIES
  PUBLIC_HEADER smtc_windows.h
  OUTPUT_NAME "smtc_windows"
)

target_compile_definitions(smtc_windows PUBLIC DART_SHARED_LIB)
