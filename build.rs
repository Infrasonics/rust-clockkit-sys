use std::path::{Path, PathBuf};
use std::io;

#[cfg(feature = "make_bindings")]
use std::env;

#[cfg(feature = "build_server")]
use std::process::Command;

#[cfg(feature = "make_bindings")]
use bindgen;

fn main() -> io::Result<()> {
    let bundle_dir = Path::new("src/vendor/clockkit/ClockKit");

    let ckfiles = vec![
        "ClockClient.cpp",
        "ClockPacket.cpp",
        "ClockServer.cpp",
        "ConfigReader.cpp",
        "PhaseLockedClock.cpp",
        "SystemClock.cpp",
        "Timestamp.cpp",
        "VariableFrequencyClock.cpp"
    ].iter().map(|f| {
        bundle_dir.join(f)
    }).collect::<Vec<PathBuf>>();

    cc::Build::new()
        .files(ckfiles)
        .cpp(true)
        .flag("--std=c++17")
        .compile("libclockkit.a");

    // Build the server for testing, unfortunately this clutters the src directory with object
    // files
    #[cfg(feature = "build_server")]
    {
    Command::new("make")
        .arg("ckserver")
        .current_dir(&bundle_dir)
        .status().unwrap();
    }

    let path_libclang = match std::env::var("LIBCLANG_PATH") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("LIBCLANG_PATH not set");
            ".".into()
        }
    };

    let _path_cppinclude = match std::env::var("CPLUS_INCLUDE_PATH") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("CPLUS_INCLUDE_PATH not set");
            ".".into()
        }
    };


    /* Left here as documentation on how the bindings were generated */
    #[cfg(feature = "make_bindings")]
    {

        let bindings = bindgen::builder()
            // std
            .opaque_type("std::.*")
            .opaque_type(".*string")
            .blocklist_function(".*String_String3")

            // kissnet
            .blocklist_function(".*endpoint_.*")

            // clockkit
            .blocklist_type(".*dur")
            .blocklist_type(".*tp")
            .header(bundle_dir.join("Clock.h").to_str().unwrap())
            .allowlist_type(".*Clock")
            .header(bundle_dir.join("PhaseLockedClock.h").to_str().unwrap())
            .allowlist_function(".*PhaseLockedClock")
            .allowlist_type(".*PhaseLockedClock")
            .allowlist_type(".*timestamp_t")
            .allowlist_type(".*_Clock")
            .allowlist_type(".*VariableFrequencyClock")
            .allowlist_type(".*ClockClient")
            .opaque_type(".*ClockClient")
            .header(bundle_dir.join("ConfigReader.h").to_str().unwrap())
            .allowlist_type(".*ConfigReader")
            .allowlist_function(".*ConfigReader.*")
            .generate_inline_functions(true)
            .disable_name_namespacing()
            .allowlist_recursively(true)
            .clang_args(&["-x", "c++",
                "--std=c++17",
                //XXX bindgen having trouble with C++ headers
                "-I", &_path_cppinclude
            ]
            ).generate()
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Unable to write bindings!");

        // Development code to update the bindings file(s) when bindgen config changes
        #[cfg(feature = "update_bindings")]
        {
        bindings.write_to_file("src/bindgen/amd64_unknown_linux_default")
            .expect("Unable to write bindings!");
        }
    }

    println!("cargo:rustc-link-search=native={}", bundle_dir.display());
    println!("cargo:rustc-link-search=native={}", path_libclang);
    println!("cargo:rustc-link-lib=static=clockkit");

    // Add dynamically linked libraries clockkit depends on
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l dylib=pthread");
    println!("cargo:rustc-flags=-l dylib=dl");
    println!("cargo:rustc-flags=-l dylib=ccgnu2");
    Ok(())

}
