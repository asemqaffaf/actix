fn main() {
    // When building for musl targets, we need specific linking directives
    #[cfg(target_env = "musl")]
    {
        // Tell cargo to link statically against PostgreSQL
        println!("cargo:rustc-link-lib=static=pq");
        
        // Tell cargo to statically link system libraries
        println!("cargo:rustc-link-arg=-static");
        
        // Add linker flag to resolve _DYNAMIC symbol issues
        println!("cargo:rustc-link-arg=-Wl,--no-dynamic-linker");
    }
}
