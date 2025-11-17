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
        let error = 0; // mumble_sys::Mumble_ErrorCode_MUMBLE_EC_OK
        let value = get_error_code_value(error);
        assert_eq!(value, 0);
        assert_eq!(mumble_sys::Mumble_ErrorCode_MUMBLE_EC_OK as i32, 0);

        let error = -1; // mumble_sys::Mumble_ErrorCode_MUMBLE_EC_GENERIC_ERROR
        let value = get_error_code_value(error);
        assert_eq!(value, -1);
    }

    #[test]
    fn test_mumble_plugin_feature_enum() {
        assert_eq!(mumble_sys::Mumble_PluginFeature_MUMBLE_FEATURE_NONE as i32, 0);
        assert_eq!(mumble_sys::Mumble_PluginFeature_MUMBLE_FEATURE_POSITIONAL as i32, 1);
        assert_eq!(mumble_sys::Mumble_PluginFeature_MUMBLE_FEATURE_AUDIO as i32, 2);
    }

    #[test]
    fn test_mumble_talking_state_enum() {
        assert_eq!(mumble_sys::Mumble_TalkingState_MUMBLE_TS_INVALID as i32, -1);
        assert_eq!(mumble_sys::Mumble_TalkingState_MUMBLE_TS_PASSIVE as i32, 0);
        assert_eq!(mumble_sys::Mumble_TalkingState_MUMBLE_TS_TALKING as i32, 1);
    }

    #[test]
    fn test_mumble_version_struct() {
        let mut version = mumble_sys::MumbleVersion { major: 0, minor: 0, patch: 0 };
        version.major = 1;
        version.minor = 2;
        version.patch = 3;

        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }
}
