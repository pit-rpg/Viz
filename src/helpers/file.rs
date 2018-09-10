use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;

pub fn find_file(dirs: &[&str], file: &str) -> Result<PathBuf, String>  {
	for dir in dirs {
		let p = Path::new(dir).join(file);
		if p.exists() {
			return Ok(p);
		}
	}
	let mut err_str = "".to_string();
	for dir in dirs {
		let p = Path::new(dir).join(file);
		err_str = format!("file not exist {};", p.to_str().unwrap());
	}
	Err(err_str)
}


pub fn read_to_string(p: &PathBuf) -> String {
	let mut f = File::open(p).expect("file not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	contents
}
