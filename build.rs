fn main() {
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=MultitouchSupport");
    }

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon/icon.ico");
        res.compile().expect("Error compiling WindowsResource");
    }
}
