fn main() {
    // Just tell cargo to link to the PostgreSQL library
    println!("cargo:rustc-link-lib=pq");
}
