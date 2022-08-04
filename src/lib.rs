use termion::{color, style};

pub fn generic_error (msg : String) {
    println!("{}{}[ Error ]{}{} {}", 
            color::Fg(color::Red),
            style::Bold,
            color::Fg(color::Reset),
            style::Reset,
            msg);
    
}

pub fn config_error (line : u16, msg : String) {
    println!("{}{}[ Config Error | Line: {} ]{}{} {}", 
            color::Fg(color::Red),
            style::Bold,
            line,
            color::Fg(color::Reset),
            style::Reset,
            msg);
}

pub fn config_warning (line : u16, msg : String) {
    println!("{}{}[ Config Warning | Line: {} ]{}{} {}", 
            color::Fg(color::Yellow),
            style::Bold,
            line,
            color::Fg(color::Reset),
            style::Reset,
            msg);
}

pub fn generic_warning (msg : String) {
    println!("{}{}[ Warning ]{}{} {}", 
            color::Fg(color::Yellow),
            style::Bold,
            color::Fg(color::Reset),
            style::Reset,
            msg);
}

pub fn info (msg : String) {
    println!("{}{}[ Info ]{}{} {}", 
            color::Fg(color::LightBlue),
            style::Bold,
            color::Fg(color::Reset),
            style::Reset,
            msg);
}

pub fn help_panel () {
    "sos";
}

pub fn find_dir(dir_find: &String, searched_dir : &String) -> bool {
    let dirs_searched_dir = std::fs::read_dir(&searched_dir).unwrap();

    for dir_in in dirs_searched_dir {
        if dir_in.unwrap().file_name().into_string().unwrap() == *dir_find {
            return true;
        }
    }
    return false;
}