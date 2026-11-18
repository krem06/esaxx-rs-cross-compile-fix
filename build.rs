#[cfg(feature = "cpp")]
fn main() {
    // `-stdlib=libc++` only applies when the *target* is macOS/iOS —
    // the original `#[cfg(target_os = "macos")]` checked the host
    // running this build script instead, so cross-compiling FROM
    // macOS TO any other target (e.g. Windows via cargo-xwin) always
    // took the Apple branch and fed a Clang-only flag to the target
    // toolchain's compiler (e.g. mingw's g++, which rejects it).
    // `CARGO_CFG_TARGET_OS` is cargo's documented way for a build
    // script to read the actual target rather than its own host.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag("-std=c++11")
        .static_crt(true)
        .file("src/esaxx.cpp")
        .include("src");
    if target_os == "macos" || target_os == "ios" {
        build.flag("-stdlib=libc++");
    }
    build.compile("esaxx");
}

#[cfg(not(feature = "cpp"))]
fn main() {}
