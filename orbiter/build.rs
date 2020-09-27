use std::env;
use std::path::PathBuf;
use chrono::Utc;

macro_rules! error {
    ($($args:tt)+) => ({
        let msg = format!($($args)*);
        println!("cargo:warning={}", msg);
        panic!(msg);
    })
}

fn main() -> Result<(), std::io::Error> {
    const ORBITER_DIR_ENV: &str = "ORBITER_DIR";
    const ORBITER_SDK_ENV: &str = "ORBITER_SDK";

    // Check target triple for MSVC 32-bit
    if env::var("TARGET").unwrap() != "i686-pc-windows-msvc" {
        error!("Orbiter plugins must use the `i686-pc-windows-msvc` target");
    }

    // Extract OrbiterSDK location
    let orbiter_sdk_path = if let Ok(sdk_path) = env::var(ORBITER_SDK_ENV) {
        PathBuf::from(sdk_path)
    } else if let Ok(orbiter_path) = env::var(ORBITER_DIR_ENV) {
        [&orbiter_path, "Orbitersdk"].iter().collect::<PathBuf>()
    } else {
        error!("{} or {} environment must be set", ORBITER_DIR_ENV, ORBITER_SDK_ENV);
    };
    let lib_path = orbiter_sdk_path.join("lib");
    let include_path = orbiter_sdk_path.join("include");

    // Check OrbiterSDK installation
    if !lib_path.join("orbiter.lib").is_file() {
        error!("{} does not contain orbiter.lib", lib_path.to_string_lossy());
    }
    if !lib_path.join("Orbitersdk.lib").is_file() {
        error!("{} does not contain Orbitersdk.lib", lib_path.to_string_lossy());
    }
    if !include_path.join("Orbitersdk.h").is_file() {
        error!("{} does not contain Orbitersdk.h", include_path.to_string_lossy());
    }

    // List of C++ files
    let header_files = [
        "src/cpp/types.h",
    ];
    let cpp_files = [
        "src/cpp/main.cpp",
        "src/cpp/module.cpp",
        "src/cpp/object.cpp",
        "src/cpp/vessel.cpp",
    ];

    // Tell Cargo when to rerun
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={}", ORBITER_DIR_ENV);
    println!("cargo:rerun-if-env-changed={}", ORBITER_SDK_ENV);
    println!("cargo:rerun-if-changed={}", lib_path.join("orbiter.lib").to_string_lossy());
    println!("cargo:rerun-if-changed={}", lib_path.join("Orbitersdk.lib").to_string_lossy());
    println!("cargo:rerun-if-changed={}", include_path.join("Orbitersdk.h").to_string_lossy());
    for file in &header_files {
        println!("cargo:rerun-if-changed={}", file);
    }
    for file in &cpp_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    // Link to Orbiter libs
    println!("cargo:rustc-link-lib=orbiter");
    println!("cargo:rustc-link-lib=Orbitersdk");
    println!("cargo:rustc-link-search={}", lib_path.to_string_lossy());

    // Build date is required by Orbiter in the ModuleDate callback
    // To make sure the current date is used, clean rebuild before
    // sending the module to the end-user.
    let now = Utc::now();
    let date = now.format("%b %e %Y");
    println!("cargo:rustc-env=ORBITER_DATE={}", date);

    // Check if statically linking with the C runtime
    let is_static_crt = env::var("CARGO_CFG_TARGET_FEATURE").map_or(false, |val| val.contains("crt-static"));
    if !is_static_crt {
        println!("cargo:warning=Your module does not link statically with the C runtime.");
        println!("cargo:warning=This means that the end-user will need to install Visual C++ redistributables.");
        println!("cargo:warning=To avoid this, add the following lines to you .cargo/config.toml file:");
        println!("cargo:warning=rustflags = [");
        println!("cargo:warning=    \"-Ctarget-feature=+crt-static\",");
        println!("cargo:warning=    \"-Clink-args=/NODEFAULTLIB:msvcrt.lib\"");
        println!("cargo:warning=]");
        println!("cargo:warning=(you might need a `cargo clean` to take the modifications into account)");
    }

    // Build the C API
    let mut cc = cc::Build::new();
    cc
        .cpp(true)
        .files(&cpp_files)
        .include(include_path)
        .include("src/cpp");

    if is_static_crt {
        cc
            .static_crt(true)
            .ar_flag("/NODEFAULTLIB:msvcrt.lib");
    }

    cc.compile("orbiter_c");

    Ok(())
}
