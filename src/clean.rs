use std::fs;
use crate::paths::get_paths;
use crate::messenger::get_msg;

pub struct Deleter {
	
}

pub fn delete_logs() -> std::io::Result<()> {
	let msg = get_msg();
	let path = get_paths();

	msg.try_delete_logs();

  fs::remove_file(&path.temp_path)?;

  Ok(())

  // println!("{}", &path.temp_path);
  // println!("delete logs module was connected");
}

pub fn delete_downloads() -> std::io::Result<()> {
	let msg = get_msg();
	let path = get_paths();

	msg.try_delete_logs();

  fs::remove_file(&path.temp_path)?;

  Ok(())

  // println!("{}", &path.temp_path);
  // println!("delete logs module was connected");
}