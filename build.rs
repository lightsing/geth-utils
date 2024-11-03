const LIB_NAME: &str = "go-geth-utils";

#[cfg(windows)]
fn main() {
    println!("cargo:rustc-link-search=native=./vendor/windows-amd64");
    println!("cargo:rustc-link-lib=static={LIB_NAME}");
}

#[cfg(not(windows))]
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Build
    let mut build = gobuild::Build::new();
    if cfg!(target_os = "macos") {
        build.ldflags("-w");
    }

    if let Err(e) = build
        .files(&["./l2geth/trace.go", "./l2geth/lib.go"])
        .try_compile(LIB_NAME)
    {
        // The error type is private so have to check the error string
        if format!("{e}").starts_with("Failed to find tool.") {
            fail(
                " Failed to find Go. Please install Go 1.18 or later \
                following the instructions at https://golang.org/doc/install.
                On linux it is also likely available as a package."
                    .to_string(),
            );
        } else {
            fail(format!("{e}"));
        }
    }

    // Files the lib depends on that should recompile the lib

    for file in &[
        "./l2geth/trace.go",
        "./l2geth/lib.go",
        "./l2geth/go.mod",
        "./l2geth/go.sum",
    ] {
        println!("cargo:rerun-if-changed={file}");
    }

    // Link
    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=static={LIB_NAME}");
}

#[cfg(not(windows))]
fn fail(message: String) {
    use std::io::{self, Write};

    let _ = writeln!(
        io::stderr(),
        "\n\nError while building geth-utils: {message}\n\n"
    );
    std::process::exit(1);
}
