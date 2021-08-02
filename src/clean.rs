use std::fs;

pub fn delete_logs() -> std::io::Result<()> {
  
  struct Paths {
    temp_path: &'static str,
  }

  let path = Paths {
    temp_path: "L:\\temp"
  };

	fs::remove_file("C:\\temp\\TEST.txt")?;
	Ok(())

  // println!("{}", &path.temp_path);
  // println!("delete logs module was connected");  

}