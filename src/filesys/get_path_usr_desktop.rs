use directories::UserDirs;

pub fn get_path() -> String {
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
	println!("User desktop path: {:?}", &tempdir);
  return tempdir;
}

pub fn get_usr_paths() -> Option<UserDirs> {
// pub fn get_usr_paths() {
  // let mut tempdir: UserDirs;
	let tempdir = UserDirs::new();

  // if let Some(user_dirs) = UserDirs::new() {
	// 	let test = user_dirs.des();
		
	// 	match test {
	// 		Some(v) => tempdir = user_dirs,
	// 		None() => println!("xz!"),
	// 	}
  // } else {
  //   println!("Some shit happened");
  // }
	// println!("User desktop path: {:?}", &tempdir);
  tempdir
}