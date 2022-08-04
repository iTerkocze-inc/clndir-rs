use termion::{color::*, style};

pub fn generic_error (msg : String) {
    println!("{}{}[ Error ]{}{} {}", 
            Fg(Red),
            style::Bold,
            Fg(Reset),
            style::Reset,
            msg);
    
}

pub fn config_error (line : u16, msg : String) {
    println!("{}{}[ Config Error | Line: {} ]{}{} {}", 
            Fg(Red),
            style::Bold,
            line,
            Fg(Reset),
            style::Reset,
            msg);
}

/*pub fn config_warning (line : u16, msg : String) {
    println!("{}{}[ Config Warning | Line: {} ]{}{} {}", 
            Fg(Yellow),
            style::Bold,
            line,
            Fg(Reset),
            style::Reset,
            msg);
}*/

pub fn generic_warning (msg : String) {
    println!("{}{}[ Warning ]{}{} {}", 
            Fg(Yellow),
            style::Bold,
            Fg(Reset),
            style::Reset,
            msg);
}

pub fn info (msg : String) {
    println!("{}{}[ Info ]{}{} {}", 
            Fg(LightBlue),
            style::Bold,
            Fg(Reset),
            style::Reset,
            msg);
}

pub fn help_panel () {
    println!(
"{}{}clndir{} {}
A directory cleaner with many features and configuration options.

{}{}USAGE:{}
    clndir [ARGS AND DIRECTORIES TO CLEAN]...

{}{}ARGS:{}
    {}-h --help       {}Display this message
    {}-v --version    {}Show version of the program
    {}-s --silent     {}Program will display no errors or warnings
    {}-d --default    {}Program will not read the configs and remain with the default values
    {}-o --output     {}Program will display more information about what it's doing
    {}-m --no-misc    {}Makes program not throw all other files not included in sorting directories",
    Fg(Green), style::Bold, style::Reset, env!("CARGO_PKG_VERSION"),
    Fg(Yellow), style::Bold, style::Reset,
    Fg(Yellow), style::Bold, style::Reset, 
    Fg(Green), style::Reset,
    Fg(Green), style::Reset,
    Fg(Green), style::Reset,
    Fg(Green), style::Reset,
    Fg(Green), style::Reset,
    Fg(Green), style::Reset,)
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