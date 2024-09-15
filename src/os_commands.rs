use std::process::Command;

pub fn commands() {
    println!("Hello From Commands");
    check_os();
    list_dir("bin");
}

fn check_os() {
    // cfg -> Evaluates Booloean Conditions at Compile Time
    let echo_cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo ayee from windows"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .args(["-c", "echo ahoy from linux"])
            .output()
            .expect("Failed to execute command")
    };
    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("Could not parse byte");
    println!("{}", cmd_output);
}

fn list_dir(dir: &str){
    let mut cmd_root = Command::new("ls");
    cmd_root.current_dir(dir).status().expect("Failed to list");
}
