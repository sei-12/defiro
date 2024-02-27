// ファイルの名前が"app_path"なのは、pathと見分けがつくようにしたかったため
// いい名前が思いつかなかった

use std::collections::VecDeque;

#[derive(PartialEq,Debug)]
pub struct AbsFilePath {
    dir: Vec<String>,
    file_name: String
}

#[derive(Debug,PartialEq)]
pub enum AbsFilePathError {
    ExpectAbsolutePath,
    FaultFileName,
    ExpectRelativePath,
    FailureGetHomedir
}

fn is_fault_file_name(file_name: &str) -> bool{
    if file_name == "." {
        return true;
    };
    if file_name == ".." {
        return true;
    };
    if file_name == "" {
        return true;
    };
    
    false
}

impl AbsFilePath {
    pub fn join_relative(&self, relative: &str) -> Result<Self,AbsFilePathError> {
        let mut splited : Vec<&str> = relative.split("/").collect();

        let front = splited.get(0).expect("bug");
        if front == &"" {
            return Err(AbsFilePathError::ExpectRelativePath);
        }

        let file_name = splited.pop().expect("bug");
        if is_fault_file_name(file_name) {
           return Err(AbsFilePathError::FaultFileName); 
        }

        let mut dirs_clone = self.dir.clone();
        
        for dir_name in splited {
            if dir_name == ".." {
                dirs_clone.pop();
                continue;
            }

            if dir_name == "." {
                continue;
            }
            
            if dir_name == "" {
                continue;
            }
            
            dirs_clone.push(dir_name.to_string())
        }

        Ok(AbsFilePath { dir: dirs_clone, file_name: file_name.to_string() })
    }

    pub fn from_string(abs_path: &str) -> Result<Self,AbsFilePathError> {
        let mut splited : VecDeque<String> = abs_path.split("/").map(|x| x.to_string()).collect();

        let root = splited.pop_front().expect("bug");
        if root != "" && root != "~" {
            return Err(AbsFilePathError::ExpectAbsolutePath);
        }
        
        if root == "~" {
            let Ok(home_dir) = std::env::var("HOME") else {
                return Err(AbsFilePathError::FailureGetHomedir);
            };
            let mut splited_home : VecDeque<String> = home_dir.split("/").map(|x| x.to_string()).collect();
            loop {
                let Some(dir_name) = splited_home.pop_back() else {
                    break;
                };
                splited.push_front(dir_name);
            }
        }


        let file_name = splited.pop_back().expect("bug");
        if is_fault_file_name(&file_name) {
           return Err(AbsFilePathError::FaultFileName); 
        }

        Ok(AbsFilePath { 
            dir: splited.into_iter().collect(),
            file_name
        })
    }

    pub fn get(&self) -> String {
        let mut jointed_dirs = self.dir.join(std::path::MAIN_SEPARATOR_STR);
        if jointed_dirs == "" {
            jointed_dirs = "/".to_string();
        }else{
            jointed_dirs = format!("/{}/",jointed_dirs)
        }
        format!("{}{}",jointed_dirs,self.file_name)
    }
}

pub fn join_or_abs(abs: &AbsFilePath, join: &str) -> Result<AbsFilePath,AbsFilePathError> {
    let mut tmp_chars = join.chars().peekable();
    let front = tmp_chars.peek();
    
    if front != Some(&'/') && front != Some(&'~') {
        abs.join_relative(join)
    }else{
        AbsFilePath::from_string(join)
    }

}

#[cfg(test)]
impl AbsFilePath {
    pub fn create_decoy() -> Self {
        AbsFilePath { dir: vec!["home".to_string()], file_name: "hello".to_string() }
    }
}

#[cfg(test)]
mod test {
    use super::{AbsFilePath, AbsFilePathError};

    #[test]
    fn join_relative() {
        test_join_relative(
            "/hello/two/three", 
            "./aaa/bbb",
            "/hello/two/aaa/bbb"
        ); 

        test_join_relative(
            "/hello/two/three", 
            "aaa/bbb",
            "/hello/two/aaa/bbb"
        );

        test_join_relative(
            "/hello/two/three", 
            ".aaa/bbb",
            "/hello/two/.aaa/bbb"
        );

        test_join_relative(
            "/hello/two/three", 
            "../aaa/bbb",
            "/hello/aaa/bbb"
        );
        test_join_relative(
            "/hello/two/three", 
            "../../aaa/bbb",
            "/aaa/bbb"
        );
        test_join_relative(
            "/hello/two/three", 
            "../../../aaa/bbb",
            "/aaa/bbb"
        );
        test_join_relative(
            "/hello/two/three", 
            "./.aaa/bbb",
            "/hello/two/.aaa/bbb"
        );
        test_join_relative(
            "/hello/two/three", 
            "./.aaa/bbb/ccc/ddd/eee/fff/ggg",
            "/hello/two/.aaa/bbb/ccc/ddd/eee/fff/ggg"
        );
        test_join_relative(
            "/hello", 
            "./aaa",
            "/aaa"
        );
        test_join_relative(
            "/hello/aaa/bbb/ccc", 
            "aaa/../bbb/ccc/../bb/sss",
            "/hello/aaa/bbb/bbb/bb/sss"
        );
        test_join_relative(
            "/hello/aaa/bbb/ccc", 
            "aaa/../bbb/ccc/../bb/sss",
            "/hello/aaa/bbb/bbb/bb/sss"
        );
        test_join_relative(
            "/hello/aaa/bbb/ccc", 
            "aaa/..//bbb//ccc/../bb/sss",
            "/hello/aaa/bbb/bbb/bb/sss"
        );
        test_join_relative(
            "/hello/aaa/bbb/ccc", 
            "aaa",
            "/hello/aaa/bbb/aaa"
        );
        test_join_relative(
            "/hello/aaa/bbb/ccc", 
            ".aaa",
            "/hello/aaa/bbb/.aaa"
        );
    }

    #[test]
    fn join_relative_err() {
        test_join_relative_err(
            "/hello",
            "./",
            AbsFilePathError::FaultFileName
        );
        test_join_relative_err(
            "/hello",
            "/aaa",
            AbsFilePathError::ExpectRelativePath
        );
        test_join_relative_err(
            "/hello",
            "aaa/",
            AbsFilePathError::FaultFileName
        );
        test_join_relative_err(
            "/hello",
            "aaa/..",
            AbsFilePathError::FaultFileName
        );
        test_join_relative_err(
            "/hello",
            "aaa/.",
            AbsFilePathError::FaultFileName
        );
        test_join_relative_err(
            "/hello",
            "",
            AbsFilePathError::ExpectRelativePath
        );
    }
    
    fn test_join_relative_err(
        from: &str,
        join: &str,
        err: AbsFilePathError
    ){
        println!("-------------------------------");
        println!("from: {}\n join:{}\n",from,join);
        let from_path = AbsFilePath::from_string(from).unwrap();
        println!("{:?}", from_path);
        let jointed_path = from_path.join_relative(join);
        assert_eq!(jointed_path.unwrap_err(),err);
    }

    fn test_join_relative(
        from: &str,
        join: &str,
        jointed: &str
    ) {
        println!("-------------------------------");
        println!("from: {}\n join:{}\n jointed:{}",from,join,jointed);
        let from_path = AbsFilePath::from_string(from).unwrap();
        println!("{:?}", from_path);
        let jointed_path = from_path.join_relative(join).unwrap();
        assert_eq!(jointed_path.get(),jointed)
    }
}