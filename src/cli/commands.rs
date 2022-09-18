use std::process::Command;
use log::{error, info};

pub fn client() -> Result<(), Box<dyn std::error::Error>> {
    
    let output = Command::new("make").arg("run-client").output()?;
    
    if !output.status.success() {
        error!("Client command failed.");
    }
    else {
        let formatted_output = String::from_utf8_lossy(&output.stdout);
        info!("COMMAND OUTPUT: {}", formatted_output);
    }
    
    Ok(())
}

pub fn server() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = Command::new("make").arg("run-server").spawn()?;
    info!("COMMAND OUTPUT: {:?}", output);
    output.wait()?;
    Ok(())
}


