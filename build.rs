fn main() {
    println!("cargo:rerun-if-changed=assets/focus-noise.ico");

    #[cfg(windows)]
    {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/focus-noise.ico");
        resource
            .compile()
            .expect("failed to embed Windows application icon");
    }
}
