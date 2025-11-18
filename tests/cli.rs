use std::process::Command as StdCommand;
use assert_cmd::Command;

#[test]
fn test_help_message() {
    let output = StdCommand::new("target/debug/mumble-server")
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
#[allow(deprecated)]
fn test_missing_ssl_error() {
    // Create a temporary directory to run the command in, ensuring no certs exist
    let temp_dir = tempfile::tempdir().unwrap();

    let mut cmd = Command::cargo_bin("mumble-server").unwrap();
    let assert = cmd
        .current_dir(temp_dir.path())
        .assert();

    let output = assert.failure();
    let error_msg_str = String::from_utf8_lossy(&output.get_output().stderr);
    assert!(error_msg_str.contains("Certificate or key file not found."));
}

#[test]
#[allow(deprecated)]
fn test_deterministic_key_generation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let cert_path = temp_dir.path().join("mumble-server.pem");
    let key_path = temp_dir.path().join("mumble-server.key");
    let hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    // Generate the first key
    let mut cmd1 = Command::cargo_bin("mumble-server").unwrap();
    let assert1 = cmd1
        .current_dir(temp_dir.path())
        .arg("--generate-keys")
        .arg("--key-from-hash")
        .arg(&hash)
        .assert();
    
    assert1.success();
    assert!(cert_path.exists());
    assert!(key_path.exists());

    let key1 = std::fs::read_to_string(&key_path).unwrap();

    // Delete the key and cert
    std::fs::remove_file(&cert_path).unwrap();
    std::fs::remove_file(&key_path).unwrap();

    // Generate the second key
    let mut cmd2 = Command::cargo_bin("mumble-server").unwrap();
    let assert2 = cmd2
        .current_dir(temp_dir.path())
        .arg("--generate-keys")
        .arg("--key-from-hash")
        .arg(&hash)
        .assert();

    assert2.success();
    assert!(cert_path.exists());
    assert!(key_path.exists());

    let key2 = std::fs::read_to_string(&key_path).unwrap();

    assert_eq!(key1, key2);
}

#[test]
fn test_invalid_hash_error() {
    let temp_dir = tempfile::tempdir().unwrap();
    let cert_path = temp_dir.path().join("cert.pem");
    let key_path = temp_dir.path().join("key.pem");
    let hash = "invalid_hash";

    let output = StdCommand::new("target/debug/mumble-server")
        .arg("--generate-keys")
        .arg("--key-from-hash")
        .arg(hash)
        .arg("--ssl-cert")
        .arg(&cert_path)
        .arg("--ssl-key")
        .arg(&key_path)
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
    let error_msg_str = String::from_utf8_lossy(&output.stderr);
    assert!(error_msg_str.contains("Invalid character 'i' at position 0"));
}
