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
    assert!(error_msg_str.contains(
        "'sslCert' and 'sslKey' must be set in the config file or via command line arguments."
    ));
}

#[test]
fn test_deterministic_key_generation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let cert_path = temp_dir.path().join("cert.pem");
    let key_path = temp_dir.path().join("key.pem");
    let hash = "a".repeat(64);

    // Generate the first key
    let output1 = Command::new("target/debug/mumble-server")
        .arg("--generate-keys")
        .arg("--key-from-hash")
        .arg(&hash)
        .arg("--ssl-cert")
        .arg(&cert_path)
        .arg("--ssl-key")
        .arg(&key_path)
        .output()
        .expect("failed to execute process");

    assert!(output1.status.success());
    assert!(cert_path.exists());
    assert!(key_path.exists());

    let key1 = std::fs::read_to_string(&key_path).unwrap();

    // Delete the key and cert
    std::fs::remove_file(&cert_path).unwrap();
    std::fs::remove_file(&key_path).unwrap();

    // Generate the second key
    let output2 = Command::new("target/debug/mumble-server")
        .arg("--generate-keys")
        .arg("--key-from-hash")
        .arg(&hash)
        .arg("--ssl-cert")
        .arg(&cert_path)
        .arg("--ssl-key")
        .arg(&key_path)
        .output()
        .expect("failed to execute process");
    
    assert!(output2.status.success());
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

    let output = Command::new("target/debug/mumble-server")
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
