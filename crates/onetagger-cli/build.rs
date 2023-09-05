fn main() {
    // Required for Python
    println!("cargo:rustc-link-arg=-Wl,-export-dynamic");

    // Set Windows icon
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("..\\..\\assets\\icon.ico");
        res.compile().unwrap();
    }
}