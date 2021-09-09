use std::{fs, io};
use crate::paths::get_paths;
use crate::messenger::get_msg;
// use std::path::Path;

// pub struct Deleter {
// 	pub custom_logs: &'static str,
// }

// impl Deleter {
// 	// pub fn delete_logs(&self) -> std::io::Result<()> {
// 	// 	let msg = get_msg();
// 	// 	let path = get_paths();
// 	// 	msg.try_delete_logs();
// 	// 	fs::remove_file(&path.temp_path)?;
// 	// 	Ok(())
// 	// }
// }

pub fn delete_logs() -> std::io::Result<()> {
	let msg = get_msg();
	let path = get_paths();

	msg.try_delete_logs();

	// println!("{:?}", &path.user_paths.download_dir());
	let entries = fs::read_dir(&path.user_paths.download_dir().unwrap())?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

	for item in entries {
		println!("{:?}", item);
	}
	

  fs::remove_file(&path.temp_path)?;

  Ok(())

  // println!("{}", &path.temp_path);
  // println!("delete logs module was connected");
}

// pub fn delete_downloads() -> std::io::Result<()> {
// 	let msg = get_msg();
// 	let path = get_paths();

// 	msg.try_delete_logs();

//   fs::remove_file(&path.temp_path)?;

//   Ok(())

//   // println!("{}", &path.temp_path);
//   // println!("delete logs module was connected");
// }