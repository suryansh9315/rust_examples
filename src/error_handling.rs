use std::process::Command;
use std::io::ErrorKind;

pub fn error_handling() {
    println!("Hello From Errors");
    list_dir("shanu");
    list_dir("bin");
}

fn list_dir(dir: &str){
    let mut cmd_root = Command::new("ls");

    // cmd_root.current_dir(dir).status().expect("Failed to list");

    // Same as except but without message
    // cmd_root.current_dir(dir).status().unwrap();

    // cmd_root.current_dir(dir).status()?;

    // This is same as the above one
    // match cmd_root.current_dir(dir).status() {
    //     Ok(cmd) => cmd,
    //     Err(e) => panic!("Error: {}", e)
    // };

    // This doesn't throws a panic and doesn't stops the execution of lines ahead
    // match cmd_root.current_dir(dir).status() {
    //     Ok(cmd) => Some(cmd),
    //     Err(e) => {
    //         println!("Directory Not Found");
    //         None
    //     }
    // };

    // Matching Error kind
    match cmd_root.current_dir(dir).status() {
        Ok(cmd) => Some(cmd),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Directory Not Found");
                None
            },
            _ => panic!("An Unexpected has occured") 
        }
    };

}