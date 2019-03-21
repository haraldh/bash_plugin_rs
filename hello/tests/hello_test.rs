use std::process::exit;

#[test]
fn test_sh() {
    println!("Hello");
    std::process::Command::new("bash").arg("test.sh").status().expect("Failed to run test.sh");
}

