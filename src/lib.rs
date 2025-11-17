#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use mumble_sys::*;

#[no_mangle]
pub extern "C" fn get_error_code_value(error_code: mumble_error_t) -> i32 {
    error_code as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_error_code_value() {
        let error = 0; // Mumble_ErrorCode::MUMBLE_EC_OK
        let value = get_error_code_value(error);
        assert_eq!(value, 0);

        let error = -1; // Mumble_ErrorCode::MUMBLE_EC_GENERIC_ERROR
        let value = get_error_code_value(error);
        assert_eq!(value, -1);
    }
}
