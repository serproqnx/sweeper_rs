pub struct Paths {
	pub temp_path: &'static str,
}

pub fn get_paths() -> Paths {

  let path = Paths {
    temp_path: "C:\\temp\\TEST.txt",
  };

	path
}