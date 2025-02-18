fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-arg=-Wl,-soname,libdora_c.so");
    }
}
