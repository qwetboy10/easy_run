use std::error::Error;
use std::io::Read;
use std::time::Instant;
use std::process::{Command,Stdio,Child};

pub fn main() {
    let mut proc = Process::new(
        String::from("ls"),
        vec![],
        None,
    );
    if let Ok(output) = proc.get_output() {
        println!("{}", output);
    }
}

struct Process {
    command_name: String,
    arguments: Vec<String>,
    started_time: Instant,
    name: Option<String>,
    proc: Child,
}

impl Process {
    pub fn new(command_name: String, arguments: Vec<String>, name: Option<String>) -> Process {
        let proc = Command::new(&command_name)
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!("Failed to run command {:?}", &command_name));
        Process {
            command_name: command_name,
            arguments: arguments,
            started_time: Instant::now(),
            name: name,
            proc: proc,
        }
    }
    
    pub fn get_output(&mut self) -> Result<String, Box<Error>> {
        let mut output = String::new();
        if let Some(stdout) = self.proc.stdout {
            &mut stdout.read_to_string(&mut output)?;
            Ok(output)
        }
        else {
            Err("Failed to capture child stdout")?
        }

    }

}
