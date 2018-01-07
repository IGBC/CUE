use std::process::Stdio;
use std::process::Command;
use std::process::Child;

struct Job {
    id: usize,
    process: Child,
    
}

pub struct Shell {
    nextJobID: usize,
    jobs: Vec<Job>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            nextJobID: 0,
            jobs: Vec::new(),
        }
    }

    pub fn exec(&mut self, cmd: &str, args: &[&str]) -> Child {
        let a = Command::new(cmd).args(args)
                 .stdout(Stdio::piped())
                 .stderr(Stdio::piped())
                 .stdin (Stdio::piped())
                 .spawn();
        
        let a = match a {
            Ok(child) => child,
            Err(error) => panic!("go fuck yourself"),
        };
        
        //let job = Job {
        //    id: self.nextJobID,
        //    process: a,
        //};
        
        //self.nextJobID += 1;
        
        //self.jobs.push(job);
        
        a
    }
}
