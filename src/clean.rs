use std::fs;
use crate::paths::get_paths;
// use crate::messages::get_msg;

pub fn delete_logs() -> std::io::Result<()> {
	
	let path = get_paths();
  
  fs::remove_file(&path.temp_path)?;

  Ok(())

  // println!("{}", &path.temp_path);
  // println!("delete logs module was connected");
}