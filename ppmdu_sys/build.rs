fn main() {
    println!("cargo:rustc-link-search=native=../ppmdu_audioutil/lib");
    println!("cargo:rustc-link-arg=-laudioutil");
    println!("cargo:rustc-link-lib=dylib=c++");

    println!("cargo:rustc-link-search=native=../out/build/arm64-macos-debug/deps/jdksmidi");
    println!("cargo:rustc-link-arg=-ljdksmidi");

    println!("cargo:rustc-link-search=native=../out/build/arm64-macos-debug/vcpkg_installed/arm64-osx/lib");
    println!("cargo:rustc-link-arg=-lPocoFoundation");
    println!("cargo:rustc-link-arg=-lpugixml");
}
