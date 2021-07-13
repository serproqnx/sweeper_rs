use directories::UserDirs;

pub fn get_path_usr_desktop() -> String {
		
	let mut tempdir = String::new();

	if let Some(user_dirs) = UserDirs::new() {
		let dsktp = user_dirs
		.desktop_dir()
		.unwrap()
		.to_str()
		.unwrap()
		.to_string();
		tempdir = dsktp;

	} else {
		println!("Some shit happened");
	}
	
	return tempdir;
	
}