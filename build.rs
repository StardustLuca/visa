use std::env;

fn main() {
    visa_name();
    visa_path();
    bindgen();
}

fn visa_name() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match (&*target_arch, &*target_os) {
        ("x86_64", "macos") => println!("cargo:rustc-link-lib=framework=VISA"),
        (_, "linux") => println!("cargo:rustc-link-lib=visa"),
        ("x86_64", _) => println!("cargo:rustc-link-lib=visa64"),
        ("x86", _) => println!("cargo:rustc-link-lib=visa32"),
        _ => unimplemented!("{}, {} are not supported", target_arch, target_os),
    }
}

fn visa_path() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match (&*target_arch, &*target_os) {
        ("x86_64", "macos") => println!("cargo:rustc-link-search=framework=/Library/Frameworks"),
        ("x86", "windows") => println!(
            r#"cargo:rustc-link-search=C:\Program Files (x86)\IVI Foundation\VISA\WinNT\lib\msc"#
        ),
        ("x86_64", "windows") => println!(
            r#"cargo:rustc-link-search=C:\Program Files (x86)\IVI Foundation\VISA\WinNT\Lib_x64\msc"#
        ),
        _ => {}
    }
}

fn bindgen() {
    let bindings = bindgen::Builder::default()
        .header("./include/visa.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate visa bindings");

    bindings
        .write_to_file(format!("./src/bindings.rs"))
        .expect("failed to write bindings to file");
}
