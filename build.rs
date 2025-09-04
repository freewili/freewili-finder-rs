use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = Config::new("freewili-finder");
    let profile = std::env::var("PROFILE").unwrap();
    let cmake_profile = match profile.as_str() {
        "debug" => "Debug",
        "release" => "Release",
        _ => panic!("Couldn't match cargo PROFILE string: {profile}"),
    };
    config
        .define("FW_BUILD_C_API", "ON")
        .define("FW_BUILD_STATIC", "ON")
        .define("FW_FINDER_BUILD_TESTS", "OFF")
        .define("FW_FINDER_ENABLE_BINDINGS_PYTHON", "OFF")
        .define("FW_BUILD_EXAMPLES", "OFF")
        .profile(cmake_profile);

    // Use Ninja generator on Windows for faster builds
    #[cfg(target_os = "windows")]
    {
        config.generator("Ninja");
        // Ensure proper C++ exception handling on Windows
        config.cxxflag("/EHsc");
        // Set proper runtime library linking
        config.define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL");
        // Force Release configuration
        config.profile("Release");
        // Build all targets, not just cfwfinder
        config.build_target("all");
    }

    // On non-Windows, build the static targets
    #[cfg(not(target_os = "windows"))]
    {
        config.build_target("cfwfinder-static");
    }

    let dst = config.build();

    // Add the library search path - adjust for Windows vs Unix
    #[cfg(target_os = "windows")]
    let lib_path = dst.join("build/lib");

    #[cfg(not(target_os = "windows"))]
    let lib_path = dst.join("build/c_api");

    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Add the main build directory for fwfinder-static
    #[cfg(not(target_os = "windows"))]
    {
        let main_build_path = dst.join("build");
        println!(
            "cargo:rustc-link-search=native={}",
            main_build_path.display()
        );
    }

    // Additional Windows-specific library paths
    #[cfg(target_os = "windows")]
    {
        let build_path = dst.join("build");
        println!("cargo:rustc-link-search=native={}", build_path.display());
        let c_api_path = dst.join("build/c_api");
        println!("cargo:rustc-link-search=native={}", c_api_path.display());
        let release_path = dst.join("build/Release");
        println!("cargo:rustc-link-search=native={}", release_path.display());
        let debug_path = dst.join("build/Debug");
        println!("cargo:rustc-link-search=native={}", debug_path.display());
    }

    // Set the runtime library path (rpath) - only on Unix systems
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());

    // Tell cargo to link the freewili-finder C API library statically
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=static=cfwfinder-static");
        println!("cargo:rustc-link-lib=static=fwfinder-static");
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=static=cfwfinder-static");
        println!("cargo:rustc-link-lib=static=fwfinder-static");
    }

    // Link platform-specific system libraries
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=setupapi");
        println!("cargo:rustc-link-lib=cfgmgr32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=ole32");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=udev");
        // Link C++ standard library for static C++ libraries
        println!("cargo:rustc-link-lib=stdc++");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=Foundation");
        // Link C++ standard library for static C++ libraries
        println!("cargo:rustc-link-lib=c++");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("freewili-finder/c_api/include/cfwfinder.h")
        .clang_arg("-Ifreewili-finder/c_api/include")
        // Enable C99 standard to support stdbool.h and bool type
        .clang_arg("-std=c99")
        // Include standard headers for bool support
        .clang_arg("-include")
        .clang_arg("stdbool.h")
        .clang_arg("-include")
        .clang_arg("stdint.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .derive_debug(true)
        .derive_copy(true)
        // Don't derive PartialEq globally to avoid function pointer comparison issues
        .derive_partialeq(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .default_alias_style(bindgen::AliasVariation::TypeAlias)
        .generate_cstr(true)
        // Add allow attribute for function pointer comparisons at the module level
        .module_raw_line("", "#![allow(unpredictable_function_pointer_comparisons)]")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Tell cargo to rerun if freewili-finder changes
    println!("cargo:rerun-if-changed=freewili-finder");

    // Note: With static linking, no DLL copying is needed!
    // The library is embedded directly in the executable.
}
