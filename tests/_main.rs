

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_main_functionality() {
        let output = Command::new("target/debug/extract_json").arg("tests/data.json").arg("name").arg("product").output().expect("Failed to execute");
    
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        assert!(output.status.success());
        assert!(stdout.contains("name"));
        assert!(stdout.contains("title"));
    }
}
