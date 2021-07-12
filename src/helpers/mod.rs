use directories::UserDirs;
// use std::any::type_name;
// use std::path::Path;

// fn type_of<T>(_: T) -> &'static str {
// 	type_name::<T>()
// }

pub fn get_path_usr_desktop() -> String {
// pub fn get_path_usr_desktop() {
		
	let mut tempdir = String::new();

	if let Some(user_dirs) = UserDirs::new() {
		let dsktp = user_dirs
		.desktop_dir()
		.unwrap()
		.to_str()
		.unwrap()
		.to_string();
		//println!("{:?}", type_of(&desktop_path));		
		tempdir = dsktp;
		//tempdir

	} else {
		println!("Some shit happened");
	}
	// type_of(tempdir);
	
	return tempdir;
	


}