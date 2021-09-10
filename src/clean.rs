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

pub fn delete_usr_downloads() -> std::io::Result<()> {
	let msg = get_msg();
	let path = get_paths();

	msg.try_delete_logs();

	let path_usr_dl = &path
		.user_paths
		.download_dir()
		.unwrap();

	let entries = fs::read_dir(path_usr_dl)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

	for item in entries {
		let is_dir = fs::metadata(&item).unwrap().is_dir();
		if is_dir == true {
			fs::remove_dir_all(&item);
		}
		// println!("is_dir: {:?}", meta.unwrap().is_dir());
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