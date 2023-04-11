#!/bin/bash

CURR_VERSION=smtc_windows-v`awk '/^version: /{print $2}' packages/smtc_windows/pubspec.yaml`

# CMake platforms (Windows)
CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/smtc_windows/windows/CMakeLists.txt
rm packages/smtc_windows/$windows/*.bak