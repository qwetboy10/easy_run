use nix::sys::select::{select, FdSet};
use nix::sys::time::{TimeVal, TimeValLike};
use std::error::Error;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::process::{Child, Command, Stdio};
use std::time::Instant;
use tabular::{row, Table};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io;

static CONT: AtomicBool = AtomicBool::new(true);

pub fn main() {
    ctrlc::set_handler(|| {
        CONT.store(false, Ordering::SeqCst);
    }).expect("Could not set signal handler");

    let mut proc_controller = ProcessController::new();

    loop {
        println!("Options: add, list, get");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        choice.pop();
        match &choice[..] {
            "add" => {
                let mut command = String::new();
                io::stdin().read_line(&mut command).unwrap();
                let mut vec = vec![];
                for part in command.split_whitespace().skip(1) {
                    vec.push(String::from(part));
                }
                command.pop();
                let proc = Process::new(command,vec, None);
                proc_controller.add(proc);
            },
            "list" => {
                println!("{}", proc_controller.list());
            },
            "get" => {
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                name.pop();
                if let Some(proc) = proc_controller.processes.iter_mut().find(|a| a.name == name) {
                    proc.print_output().unwrap();
                }
                else {
                    println!("There is no process of that name");
                }
            }
            _ => break
        }
    }

}

struct ProcessController {
    processes: Vec<Process>,
}

impl ProcessController {
    pub fn new() -> ProcessController {
        ProcessController { processes: vec![] }
    }
    pub fn add(&mut self, p: Process) {
        self.processes.push(p);
    }
    pub fn list(&self) -> String {
        let mut table = Table::new("{:<} {:<} {:<}");
        table.add_row(row!("Name", "Uptime", "Original Command"));
        for proc in &self.processes {
            let duration = Instant::now() - proc.started_time;
            let mut original_command = proc.command_name.clone();
            original_command.push(' ');
            original_command.push_str(&proc.arguments.join(" "));
            table.add_row(row!(
                &proc.name,
                format!("{:3?}", duration),
                original_command
            ));
        }
        return format!("{}", table);
    }
}

struct Process {
    command_name: String,
    arguments: Vec<String>,
    started_time: Instant,
    name: String,
    proc: Child,
}

impl Process {
    pub fn new(command_name: String, arguments: Vec<String>, name: Option<String>) -> Process {
        let proc = Command::new(&command_name)
            .args(&arguments)
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!("Failed to run command {:?}", &command_name));
        Process {
            command_name: command_name.clone(),
            arguments: arguments,
            started_time: Instant::now(),
            name: name.unwrap_or(command_name),
            proc: proc,
        }
    }

    pub fn print_output(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(stdout) = &mut self.proc.stdout {
            while CONT.load(Ordering::SeqCst) {
                let fd = stdout.as_raw_fd();
                let mut fdset = FdSet::new();
                fdset.insert(fd);
                let mut timeout = TimeVal::seconds(1);
                let fd_ready = select(None, &mut fdset, None, None, &mut timeout)?;
                //the fd has some data to read
                if fd_ready == 1 {
                    let mut buf = [0; 1024 * 8];
                    let num_bytes = stdout.read(&mut buf[..])?;
                    let string = String::from_utf8_lossy(&buf[..num_bytes]).into_owned();
                    print!("{}", string);
                    if num_bytes < 1024 * 8 || num_bytes == 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
            return Ok(());
        } else {
            Err("Failed to capture child stdout")?
        }
    }
}
