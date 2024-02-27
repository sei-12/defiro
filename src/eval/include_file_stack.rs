use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct IncludeFileStackFault {
    
}

pub struct IncludeFileStack {
    included_files: Vec<String>,
}

impl IncludeFileStack {
    pub fn new() -> Self {
        Self { included_files: Vec::new() }
    }
    pub fn pop(&mut self){
        self.included_files.pop();
    } 
    
    pub fn push(&mut self, path: PathBuf) -> Result<(),IncludeFileStackFault> {
        let absolute_file_path = match fs::canonicalize(path) {
            Ok(path) => match path.to_str() {
                Some(str) => str.to_string(),
                None => {
                    return Err(IncludeFileStackFault {  });
                }
            },
            Err(_) => {
                return Err(IncludeFileStackFault {  });
            }
        };


        let exist = self.included_files.iter().find(|&path| path == &absolute_file_path).is_some();       

        if exist {
            Err(IncludeFileStackFault{})
        }else{
            self.included_files.push(absolute_file_path);
            Ok(())            
        }
    }
}
