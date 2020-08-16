#[deny(unsafe_code)]
use std::env;

#[cfg(feature = "bindgen")]
use bindgen;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod paths {
    pub const HEADER: &str = "vendor/linux/x64/ftd2xx.h";
    pub const SEARCH: &str = "vendor/linux/x64/build";
}

#[cfg(all(target_os = "linux", target_arch = "x86"))]
mod paths {
    pub const HEADER: &str = "vendor/linux/x86/ftd2xx.h";
    pub const SEARCH: &str = "vendor/linux/x86/build";
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod paths {
    pub const HEADER: &str = "vendor/windows/ftd2xx.h";
    pub const SEARCH: &str = "vendor/windows/amd64";
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
mod paths {
    pub const HEADER: &str = "vendor/windows/ftd2xx.h";
    pub const SEARCH: &str = "vendor/windows/i386";
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let mut header = cwd.clone();
    header.push(paths::HEADER);
    let mut search = cwd.clone();
    search.push(paths::SEARCH);

    println!(
        "cargo:rustc-link-search=native={}",
        search.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=ftd2xx");
    println!("cargo:rerun-if-changed={}", header.to_str().unwrap());

    #[cfg(feature = "bindgen")]
    {
        let bindings = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .whitelist_function("FT_.*")
            .whitelist_type("FT_.*")
            .whitelist_var("FT_.*")
            .rustfmt_bindings(true)
            .derive_default(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
