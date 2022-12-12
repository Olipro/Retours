
fn main() {
    let detours = cmake::Config::new(".")
        .profile("RelWithDebInfo")
        .build();

    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rustc-link-search=native={}", detours.display());
    println!("cargo:rustc-link-lib=static=lib/detours");
}