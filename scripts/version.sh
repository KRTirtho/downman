#!/bin/bash

CURR_VERSION=downman-v`awk '/^version: /{print $2}' packages/downman/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/downman/ios/downman.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/downman/macos/downman.podspec
rm packages/downman/macos/*.bak packages/downman/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/downman/$CMAKE_PLATFORM/CMakeLists.txt
    rm packages/downman/$CMAKE_PLATFORM/*.bak
done

git add packages/downman/
