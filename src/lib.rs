#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use mumble_sys::*;

pub mod cli;
pub mod config;
pub mod connection;
pub mod db;
pub mod embed;
pub mod lan;
pub mod local;
pub mod public;
pub mod server;
pub mod ui;

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
        assert_eq!(
            mumble_sys::Mumble_PluginFeature_MUMBLE_FEATURE_NONE as i32,
            0
        );
        assert_eq!(
            mumble_sys::Mumble_PluginFeature_MUMBLE_FEATURE_POSITIONAL as i32,
            1
        );
        assert_eq!(
            mumble_sys::Mumble_PluginFeature_MUMBLE_FEATURE_AUDIO as i32,
            2
        );
    }

    #[test]
    fn test_mumble_talking_state_enum() {
        assert_eq!(mumble_sys::Mumble_TalkingState_MUMBLE_TS_INVALID as i32, -1);
        assert_eq!(mumble_sys::Mumble_TalkingState_MUMBLE_TS_PASSIVE as i32, 0);
        assert_eq!(mumble_sys::Mumble_TalkingState_MUMBLE_TS_TALKING as i32, 1);
    }

    #[test]
    fn test_mumble_version_struct() {
        let mut version = mumble_sys::MumbleVersion {
            major: 0,
            minor: 0,
            patch: 0,
        };
        version.major = 1;
        version.minor = 2;
        version.patch = 3;

        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_mumble_transmission_mode_enum() {
        assert_eq!(
            mumble_sys::Mumble_TransmissionMode_MUMBLE_TM_CONTINOUS as i32,
            0
        );
        assert_eq!(
            mumble_sys::Mumble_TransmissionMode_MUMBLE_TM_VOICE_ACTIVATION as i32,
            1
        );
        assert_eq!(
            mumble_sys::Mumble_TransmissionMode_MUMBLE_TM_PUSH_TO_TALK as i32,
            2
        );
    }

    #[test]
    fn test_mumble_error_code_enum() {
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_INTERNAL_ERROR as i32,
            -2
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_GENERIC_ERROR as i32,
            -1
        );
        assert_eq!(mumble_sys::Mumble_ErrorCode_MUMBLE_EC_OK as i32, 0);
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_POINTER_NOT_FOUND as i32,
            1
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_NO_ACTIVE_CONNECTION as i32,
            2
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_USER_NOT_FOUND as i32,
            3
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_CHANNEL_NOT_FOUND as i32,
            4
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_CONNECTION_NOT_FOUND as i32,
            5
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_UNKNOWN_TRANSMISSION_MODE as i32,
            6
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_AUDIO_NOT_AVAILABLE as i32,
            7
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_INVALID_SAMPLE as i32,
            8
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_INVALID_PLUGIN_ID as i32,
            9
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_INVALID_MUTE_TARGET as i32,
            10
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_CONNECTION_UNSYNCHRONIZED as i32,
            11
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_INVALID_API_VERSION as i32,
            12
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_UNSYNCHRONIZED_BLOB as i32,
            13
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_UNKNOWN_SETTINGS_KEY as i32,
            14
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_WRONG_SETTINGS_TYPE as i32,
            15
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_SETTING_WAS_REMOVED as i32,
            16
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_DATA_TOO_BIG as i32,
            17
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_DATA_ID_TOO_LONG as i32,
            18
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_API_REQUEST_TIMEOUT as i32,
            19
        );
        assert_eq!(
            mumble_sys::Mumble_ErrorCode_MUMBLE_EC_OPERATION_UNSUPPORTED_BY_SERVER as i32,
            20
        );
    }

    #[test]
    fn test_mumble_positional_data_error_code_enum() {
        assert_eq!(
            mumble_sys::Mumble_PositionalDataErrorCode_MUMBLE_PDEC_OK as i32,
            0
        );
        assert_eq!(
            mumble_sys::Mumble_PositionalDataErrorCode_MUMBLE_PDEC_ERROR_TEMP as i32,
            1
        );
        assert_eq!(
            mumble_sys::Mumble_PositionalDataErrorCode_MUMBLE_PDEC_ERROR_PERM as i32,
            2
        );
    }

    #[test]
    fn test_mumble_settings_key_enum() {
        assert_eq!(mumble_sys::Mumble_SettingsKey_MUMBLE_SK_INVALID as i32, -1);
        assert_eq!(
            mumble_sys::Mumble_SettingsKey_MUMBLE_SK_AUDIO_INPUT_VOICE_HOLD as i32,
            0
        );
        assert_eq!(
            mumble_sys::Mumble_SettingsKey_MUMBLE_SK_AUDIO_INPUT_VAD_SILENCE_THRESHOLD as i32,
            1
        );
        assert_eq!(
            mumble_sys::Mumble_SettingsKey_MUMBLE_SK_AUDIO_INPUT_VAD_SPEECH_THRESHOLD as i32,
            2
        );
        assert_eq!(
            mumble_sys::Mumble_SettingsKey_MUMBLE_SK_AUDIO_OUTPUT_PA_MINIMUM_DISTANCE as i32,
            3
        );
        assert_eq!(
            mumble_sys::Mumble_SettingsKey_MUMBLE_SK_AUDIO_OUTPUT_PA_MAXIMUM_DISTANCE as i32,
            4
        );
        assert_eq!(
            mumble_sys::Mumble_SettingsKey_MUMBLE_SK_AUDIO_OUTPUT_PA_BLOOM as i32,
            5
        );
        assert_eq!(
            mumble_sys::Mumble_SettingsKey_MUMBLE_SK_AUDIO_OUTPUT_PA_MINIMUM_VOLUME as i32,
            6
        );
    }
}
