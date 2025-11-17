use std::process::Command;

#[test]
fn test_help_message() {
    let output = Command::new("target/debug/mumble-server")
        .arg("--help")
        .output()
        .expect("failed to execute process");

    let help_msg_str = String::from_utf8_lossy(&output.stdout);
    assert!(help_msg_str.contains("Mumble server (murmur)"));
    assert!(help_msg_str.contains("Usage: mumble-server [OPTIONS]"));
    assert!(help_msg_str.contains("--help"));
    assert!(help_msg_str.contains("--version"));
}

#[test]
fn test_missing_ssl_error() {
    let output = Command::new("target/debug/mumble-server")
        .output()
        .expect("failed to execute process");

    let error_msg_str = String::from_utf8_lossy(&output.stderr);
    assert!(error_msg_str.contains("'sslCert' and 'sslKey' must be set in the config file or via command line arguments."));
}
