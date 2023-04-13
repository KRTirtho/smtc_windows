$BUILD_DIR = "platform-build"

if (Test-Path $BUILD_DIR) {
    Remove-Item -Recurse -Force $BUILD_DIR
}

New-Item -ItemType Directory -Path $BUILD_DIR

Set-Location $BUILD_DIR

cargo install cargo-xwin

function win_build {
    $TARGET = $args[0]
    $PLATFORM_NAME = $args[1]
    $LIBNAME = $args[2]
    
    rustup target add $TARGET
    cargo xwin build --lib --target $TARGET -r
    New-Item -ItemType Directory -Path $PLATFORM_NAME
    Copy-Item "../target/$TARGET/release/$LIBNAME" $PLATFORM_NAME/
}

$WINDOWS_LIBNAME = "smtc_windows.dll"

win_build "aarch64-pc-windows-msvc" "windows-arm64" $WINDOWS_LIBNAME

win_build "x86_64-pc-windows-msvc" "windows-x64" $WINDOWS_LIBNAME


tar -czvf other.tar.gz windows-*

Remove-Item -Recurse -Force windows-*