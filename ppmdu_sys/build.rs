fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR");

    println!("cargo:rustc-link-search=native={dir}/../ppmdu_audioutil/lib");
    println!("cargo:rustc-link-lib=audioutil");
    println!("cargo:rustc-link-lib=dylib=c++");

    println!("cargo:rustc-link-search=native={dir}/../out/build/arm64-macos-debug/deps/jdksmidi");
    println!("cargo:rustc-link-lib=jdksmidi");

    println!("cargo:rustc-link-search=native={dir}/../out/build/arm64-macos-debug/vcpkg_installed/arm64-osx/lib");
    println!("cargo:rustc-link-lib=PocoFoundation");
    println!("cargo:rustc-link-lib=pugixml");
}
