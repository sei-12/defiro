use crate::{app_path::AbsFilePath, fault};

#[derive(Debug)]
pub struct IncludeFileStackFault {
    path: String,
}

impl fault::Fault for IncludeFileStackFault {
    fn msg(&self) -> String {
        format!(
            "IncludeError: Recursive inclusion detected. : {}",
            self.path
        )
    }
}

pub struct IncludeFileStack {
    included_files: Vec<AbsFilePath>,
}

impl IncludeFileStack {
    pub fn new() -> Self {
        Self {
            included_files: Vec::new(),
        }
    }
    pub fn pop(&mut self) {
        self.included_files.pop();
    }

    pub fn push(&mut self, abs_path: AbsFilePath) -> Result<(), IncludeFileStackFault> {
        let exist = self
            .included_files
            .iter()
            .find(|&path| path == &abs_path)
            .is_some();

        if exist {
            Err(IncludeFileStackFault {
                path: abs_path.get(),
            })
        } else {
            self.included_files.push(abs_path);
            Ok(())
        }
    }

    pub fn get_current_file(&self) -> &AbsFilePath {
        self.included_files.last().expect("bug")
    }
}
