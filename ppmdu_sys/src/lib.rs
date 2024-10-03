use std::ffi::{c_char, CString};

extern "C" {
    fn export_audio(input_path: *const c_char, output_path: *const c_char) -> bool;
}

pub fn export_all_audio(input_path: &str, output_path: &str) {
    let input_path = CString::new(input_path).expect("cannot convert path to char*");
    let output_path = CString::new(output_path).expect("cannot convert path to char*");

    unsafe {
        export_audio(input_path.as_ptr(), output_path.as_ptr());
    }
}
