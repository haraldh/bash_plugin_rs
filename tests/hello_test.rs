#[test]
fn test_sh() {
    println!("Hello");
    assert!(std::process::Command::new("bash")
        .arg("tests/hello_test.sh")
        .status()
        .expect("Failed to run test.sh")
        .success());
}
