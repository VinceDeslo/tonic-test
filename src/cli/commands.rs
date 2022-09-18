use std::process::Command;
    
pub fn client() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("make").arg("run-client").output()?;
    
    if !output.status.success() {
        println!("Curl command failed.");
    }
    else {
        let formatted_output = String::from_utf8_lossy(&output.stdout);
        println!("COMMAND OUTPUT: {}", formatted_output);
    }
    
    Ok(())
}


