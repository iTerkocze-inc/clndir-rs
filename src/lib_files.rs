use std::process::Command;

pub fn find_dir(dir_find: &String, searched_dir: &String) -> bool {
  let dirs_searched_dir = std::fs::read_dir(&searched_dir).unwrap();

  for dir_in in dirs_searched_dir {
    if dir_in.unwrap().file_name().into_string().unwrap() == *dir_find {
      return true;
    }
  }
  return false;
}

pub fn find_pattern(re_inp: String, mut to_find: String) -> bool {
  to_find = String::from(to_find.split(".").next().unwrap());

  let re = regex::Regex::new(re_inp.trim_start_matches("\"").trim_end_matches("\"")).unwrap();
  let re_result = re.find(to_find.as_str());

  match re_result {
    None => return false,
    _ => return true,
  }
}

pub fn move_file(file_path: String, move_to_path: String) {
  let _ = Command::new("mv").args([file_path, move_to_path]).spawn();
}
