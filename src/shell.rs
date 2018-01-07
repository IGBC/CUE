use std::process::*;
use std::collections::HashMap;

pub struct Job {
    id: usize,
    process: Child,
}

pub struct Shell {
    nextJobID: usize,
    jobs: HashMap<usize, Job>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            nextJobID: 0,
            jobs: HashMap::new(),
        }
    }

    pub fn exec(&mut self, cmd: &str, args: &[&str]) -> usize {
        let a = Command::new(cmd).args(args)
                 .stdout(Stdio::piped())
                 .stderr(Stdio::piped())
                 .stdin (Stdio::piped())
                 .spawn();
        
        let a = a.unwrap();
        
        let jobID = self.nextJobID;
        
        let job = Job {
            id: jobID,
            process: a,
        };
        
        self.nextJobID += 1;
        
        self.jobs.insert(jobID, job);
        
        jobID
    }
    
    pub fn get_stdout(&mut self, job: usize) -> &mut ChildStdout {
        // Could panic at any moment...
        (&mut (&mut self.jobs.get_mut(&job).unwrap().process).stdout).as_mut().unwrap()
    }
}
