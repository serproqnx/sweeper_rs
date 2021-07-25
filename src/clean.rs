
pub fn delete_logs() {
  
  struct Paths {
    temp_path: String,
  }

  let path = Paths {
    temp_path: "some ".to_string()
  };

  println!("{}", &path.temp_path);

  println!("delete logs module was connected");  

}