use std::{fmt::format, process::Command}; // Command::new("the &command").spawn();
use users::get_current_username;

// All the functions used to print warnings, errors and infos
mod lib_files;
mod lib_text;

#[derive(Clone)]
struct SortingTypes {
  by_name: String,
  by_format: String,
  by_date: String,
}

struct SortingDirectory {
  dir_name: String,
  sorting: SortingTypes,
}

fn main() {
  // Gets the user's name
  let mut uname = String::from("");
  match get_current_username() {
    Some(temp_uname) => {
      uname = temp_uname
        .into_string()
        .unwrap_or(String::from("Unknown error"))
    }
    None => println!("The current user does not exist"),
  }

  // Creates a string which is the path to user's home directory
  let home_path: String = format!("/home/{uname}");
  drop(uname);

  // All the variables set with configs
  let mut archive_path: String = format!("{}/Archive", home_path);
  let mut misc_dir: String = String::from("Misc");
  let mut sorting_directories : Vec<SortingDirectory> = vec![
      SortingDirectory {
        dir_name : String::from("Videos"),
        sorting : SortingTypes {
          by_name : String::from(""),
          by_format : String::from("mp4 webm mkv vob ogv ogg avi mov viv amv m4p m4v mpg mp2 mpeg mpe mpv m2v m4v"),
          by_date : String::from("")
        }
      },SortingDirectory {
        dir_name : String::from("Pictures"),
        sorting : SortingTypes {
          by_name : String::from(""),
          by_format : String::from("jpg png gif mng jfif exif tiff bmp ppm pgm pbm pnm webp heif avif bat cd5 clip cpt kra mdp pdn psd sai xcf svg svgz bmp"),
          by_date : String::from("")
        }
      },SortingDirectory {
        dir_name : String::from("Archive files"),
        sorting : SortingTypes {
          by_name : String::from(""),
          by_format : String::from("rar 7z jar a ar iso tar br bz2 gz ls lz4 lzma lzo rz sz xz z apk dmg pea zipx zz"),
          by_date : String::from("")
        }
      },SortingDirectory {
        dir_name : String::from("Documents"),
        sorting : SortingTypes {
          by_name : String::from(""),
          by_format : String::from("doc pdf md odt xml html sxw htm tex txt gdoc docm epub ott rtf uot fodt docx xhtml xml xls wmf tif tiff odp otp odg fodp uop pptx ppsx potx ppt pps pot pptm odf mml"),
          by_date : String::from("")
        }
      },SortingDirectory {
        dir_name : String::from("Audio files"),
        sorting : SortingTypes {
          by_name : String::from(""),
          by_format : String::from("mp3 wav ogg aiff oga mogg awb 3gp aac flac"),
          by_date : String::from("")
        }
      }
  ];

  // ##########################################
  // This section sets all the command-line arguments
  // ##########################################

  // All options that can be set up with arguments
  let mut is_silent_mode: bool = false;
  let mut is_output_mode: bool = false;
  let mut is_default_mode: bool = false;
  let mut is_sorting_misc: bool = true;

  let mut is_priority_name_sorting: bool = false;
  let mut is_priority_date_sorting: bool = false;
  let mut is_only_name: bool = false;
  let mut is_only_format: bool = false;
  let mut is_only_date: bool = false;

  let mut dirs_to_clear: Vec<String> = vec![];
  let mut config_path: String = format!("{}/.config/clndir/config.conf", home_path);

  // Gets all the arguments
  let mut args: Vec<String> = std::env::args().collect();
  args.remove(0);

  // Argument handling
  for arg in args {
    // gets first two letters of the argument in order to check if it's a long or short argument
    let arg_start: &str = &arg[..2];

    // Checks if the argument is long
    if arg_start == "--" {
      // Pattern matching for long arguments
      let arg_to_match = &arg.trim_start_matches("--");
      match *arg_to_match {
        "silent" => is_silent_mode = true,
        "output" => is_output_mode = true,
        "default" => is_default_mode = true,
        "no-misc" => is_sorting_misc = false,
        "name-sorting" => is_priority_name_sorting = true,
        "last-modified-sorting" => is_priority_date_sorting = true,
        "only-name" => is_only_name = true,
        "only-format" => is_only_format = true,
        "only-modified-sorting" => is_only_date = true,
        "help" => {
          lib_text::help_panel();
          return;
        }
        "version" => {
          lib_text::version_panel();
          return;
        }
        _ => {
          if arg_to_match.starts_with("config=") {
            config_path = String::from(arg_to_match.trim_start_matches("config="));
          }
          else if !is_silent_mode {
            lib_text::generic_error(format!("Unknown flag: \"{}\"", arg_to_match));
            return;
          }
        }
      }
    } else if arg.chars().next().unwrap() == '-' {
      // Pattern matching for short arguments
      let split_args = arg.trim_start_matches('-').chars();
      for one_arg in split_args {
        match one_arg {
          's' => is_silent_mode = true,
          'o' => is_output_mode = true,
          'd' => is_default_mode = true,
          'm' => is_sorting_misc = false,
          'n' => is_priority_name_sorting = true,
          'h' => {
            lib_text::help_panel();
            return;
          }
          'v' => {
            lib_text::version_panel();
            return;
          }
          _ => {
            if !is_silent_mode {
              lib_text::generic_error(format!("Unknown flag: \"{}\"", one_arg));
            }
            return;
          }
        }
      }
    } else {
      dirs_to_clear.push(arg);
    }
  }

  // Checks if any of the is_only_* flags has been set
  if is_only_date | is_only_format | is_only_name {
    // Cancels out the sorting type priority flags
    is_priority_date_sorting = false;
    is_priority_name_sorting = false;
  
    // This counts how many of these flags have been used - if more than one the program will
    // print an error message and quit
    let mut amount_is_only : u8 = 0;
    if is_only_date { amount_is_only += 1; }
    if is_only_format { amount_is_only += 1; }
    if is_only_name { amount_is_only += 1; }

    if amount_is_only > 1 {
      if !is_silent_mode {
        lib_text::generic_error(String::from("You can use only one flag at a time that specifies what sorting the program will use"));
      }
      return;
    }
  }

  // Checks if both sorting priority flags have been used - if yes prints an error and quits
  if is_priority_date_sorting & is_priority_name_sorting {
    if !is_silent_mode {
      lib_text::generic_error(String::from("You can use only one flag at a time that specifies which sorting type the program will prioritize"));
    }
    return;
  }

  // Cancels out the output mode if silent mode is selected
  if is_silent_mode {
    is_output_mode = false;
  };

  // ##########################################
  // This section reads the config file
  // ##########################################

  if !is_default_mode {
    let conf_txt =
      std::fs::read_to_string(&config_path)
        .unwrap_or(String::from(""));
    if conf_txt != "" {
      let conf_lines = conf_txt.split('\n');
      let mut directories_declaration: bool = false;
      let mut did_clean: bool = false;
      let mut current_line: u16 = 0;
      'conf_lines_loop: for line in conf_lines {
        current_line += 1;
        if line.len() != 0 {
          // Checks if the line isn't empty
          // Skips the line on comment
          if line.chars().next().unwrap() == '#' {
            continue;
          }

          // Sets the direcotires declaration mode
          if line == "[Directories]" {
            directories_declaration = true;
            continue;
          }

          let temp_parts = line.split_once('=');
          let parts;
          match temp_parts {
            None => {
              if !is_silent_mode {
                lib_text::config_error(
                  current_line,
                  String::from(
                    "Value has to be assigned to a parameter with sign '='",
                  ),
                )
              }
              return;
            }
            _ => parts = temp_parts.unwrap(),
          }

          // The parameter
          let param = parts.0.trim();

          // Value set to the parameter
          let val = String::from(parts.1.trim().trim_matches('"').trim_matches('\''));

          // Checks if there was some value given to the parameter
          if val == "" {
            if !is_silent_mode {
              lib_text::config_error(
                current_line,
                format!("No value assigned to the parameter: \"{}\"", param),
              );
            }
            return;
          }

          drop(parts);

          if directories_declaration {
            // temporary sorting format variable for checking if the format is specified
            let temp_sorting_format = val.split_once(" ");

            // The sorting format variable that will be used later
            let sorting_format: &str;

            // Checks if the format is specified
            match temp_sorting_format {
              None => {
                if val != "*" {
                  if !is_silent_mode {
                    lib_text::config_error(
                      current_line,
                      format!("No value given to parameter: \"{}\"", param),
                    )
                  }
                  return;
                } else {
                  sorting_format = "formats";
                }
              }
              _ => sorting_format = temp_sorting_format.unwrap().0,
            }

            // Checks if it has marked that the default sorting directories were cleared so it
            // doesn't do that again
            if !did_clean {
              sorting_directories.clear();
              did_clean = true;
            }

            // Creates the misc directory if the argument is *
            if val == "*" {
              misc_dir = String::from(param);
              continue; // Continues to another loop cycle so it doesn't add misc dir to others
            }

            // Sorting by file format
            if sorting_format == "names" {
              for sr_dir in &mut sorting_directories {
                if sr_dir.dir_name == param {
                  sr_dir.sorting.by_name = val.clone();
                  break 'conf_lines_loop;
                }
              }

              let temp_sorting_dir = SortingDirectory {
                dir_name: String::from(param),
                sorting: SortingTypes {
                  by_name: val,
                  by_format: String::from(""),
                  by_date : String::from("")
                },
              };
              sorting_directories.push(temp_sorting_dir);
            }
            // Sorting by file name
            else if sorting_format == "formats" {
              for sr_dir in &mut sorting_directories {
                if sr_dir.dir_name == param {
                  sr_dir.sorting.by_format = val.clone();
                  break 'conf_lines_loop;
                }
              }

              let temp_sorting_dir = SortingDirectory {
                dir_name: String::from(param),
                sorting: SortingTypes {
                  by_name: String::from(""),
                  by_format: val,
                  by_date : String::from("")
                },
              };
              sorting_directories.push(temp_sorting_dir);
            }
            // Sorting by last modification date
            else if sorting_format == "modification" {
              let val_time : String = val.clone();

              if val_time.split(":").count() != 3 {
                if !is_silent_mode {
                  lib_text::config_error(current_line, String::from("Invalid modification date specifier. Modification date specifier should be in this format \"(< or >):(hours):(minutes):(seconds)\" and should be separated by spaces from other speifiers."));
                }
                return;
              }

              for sr_dir in &mut sorting_directories {
                // Modification date specifier
                if sr_dir.dir_name == param {
                  sr_dir.sorting.by_date = val_time;
                  break 'conf_lines_loop;
                }
              }

              let temp_sorting_dir = SortingDirectory {
                dir_name : String::from(param),
                sorting : SortingTypes {
                  by_name: String::from(""),
                  by_format: String::from(""),
                  by_date: val_time
                },
              };
              sorting_directories.push(temp_sorting_dir);
            }
            // If the sorting format is invalid
            else {
              if !is_silent_mode {
                lib_text::config_error(
                  current_line,
                  format!("Invalid sorting format: \"{}\"", sorting_format),
                )
              }
              return;
            } // If wrong sorting type specified
          } // If directories declaration

          else {
            // Sets the custom archive path
            match param {
              "archive_path" => archive_path = val,
              "misc" => {
                if val == "false" {
                  is_sorting_misc = false;
                }
              }
              _ => {
                if !is_silent_mode {
                  lib_text::config_error(
                    current_line,
                    format!("Unknown parameter: \"{}\"", param),
                  );
                }
                return;
              } // If wrong parameter
            }; // Match param
          } // If not directories declaration
        }
      }
    } 
    else if is_output_mode {
      // In output mode prints a warning that there's not config file
      lib_text::generic_warning(format!("No configs detected in {}", config_path));
    }
    drop(conf_txt);
  } // If is default mode

  else if is_output_mode {
    lib_text::info(String::from("Default mode is on so configs weren't readen"));
  }

  // ##########################################
  // This section creates all the directories in path of the archive_path
  // ##########################################

  let mut current_dir: String = String::from('/');
  for dir in archive_path.trim_matches('/').split('/') {
    // If the searched directory has been found in current directory it stops the loop
    if lib_files::find_dir(&String::from(dir), &current_dir.clone()) {
      current_dir = format!("{}{}/", current_dir, dir);
      continue;
    }

    // Appends another directory anyway to the current directory
    current_dir = format!("{}{}/", current_dir, dir);

    if is_output_mode {
      lib_text::info(format!(
        "Directory \"{}\" not found. Creating it in \"{}\"",
        dir, current_dir
      ));
    }

    // If the folder wasn't found in current directory then this creates it
    drop(std::fs::create_dir(&current_dir));
  }
  drop(current_dir);

  // ##########################################
  // This section creates the sorting directories
  // ##########################################

  let mut is_empty: bool = true;
  for sr_dir in &sorting_directories {
    let mut dir_found: bool = false;
    let sr_dirs_in = std::fs::read_dir(format!("{}/", &archive_path)).unwrap();

    for dir_searching in sr_dirs_in {
      // If this line isn't executed this means that there aren't any dirs
      is_empty = false;

      // If current checked directory is same name as current checked sorting dir it marks that
      if dir_searching.unwrap().file_name().into_string().unwrap() == sr_dir.dir_name {
        if is_output_mode {
          lib_text::info(format!(
            "Directory \"{}\" already found in the archive",
            sr_dir.dir_name
          ))
        }

        dir_found = true;
        break;
      }
    }

    // If the archive directory is empty it creates all the sorting directories
    if is_empty {
      if is_output_mode {
        lib_text::info(String::from("The archive directory is empty. Creating all the sorting directories without checking"));
      }

      for sr_dir in &sorting_directories {
        drop(std::fs::create_dir(&format!(
          "{}/{}",
          &archive_path, &sr_dir.dir_name
        )));
      }
      break;
    }

    // If the sorting directoy wasn't found this creates it
    if !dir_found {
      if is_output_mode {
        lib_text::info(format!(
          "Directory \"{}\" not found in the archive directory. Creating \"{}\"",
          sr_dir.dir_name, sr_dir.dir_name
        ));
      }

      // Creates the sorting directory that wasn't found in the archive
      drop(std::fs::create_dir(&format!(
        "{}/{}",
        &archive_path, &sr_dir.dir_name
      )));
    }
  }

  // ##########################################
  // This section moves the files
  // ##########################################

  // If there isn't any directory to clean specified by the use it pushes "." which means current directory that the
  // user is working in
  if dirs_to_clear.len() == 0 {
    dirs_to_clear.push(String::from("."));
  }

  let sr_dirs_arc = std::sync::Arc::new(sorting_directories);
  let archive_path_arc = std::sync::Arc::new(archive_path.clone());
  let misc_dir_arc = std::sync::Arc::new(misc_dir);

  // This loop goes through all the directories that have to be cleared and everything else going on in this section
  // will be happening in this for loop
  for current_dir in dirs_to_clear {
    // Gets all the files in current directory
    let files_result = std::fs::read_dir(&current_dir);

    // Declares the variable that contains all the files to be moved before the match so it can be used later on
    // outside of the match's scope
    let files: std::fs::ReadDir;

    // Either retrieves the file or shows an error that the path is invalid
    match files_result {
      Ok(v) => files = v,
      _ => {
        if !is_silent_mode {
          lib_text::generic_error(format!("No directory found at path: {}", current_dir));
        }
        return;
      }
    }

    // Formats the current path string so the program is sure about it's format
    let current_dir_arc =
      std::sync::Arc::new(String::from(current_dir.trim_start_matches("/")));

    // Handles to manage threading
    let mut handles = vec![];

    // Goes through every file and moves them
    for file in files {
      let sr_dirs_arc_clone = sr_dirs_arc.clone();
      let archive_path_arc_clone = archive_path_arc.clone();
      let current_dir_arc_clone = current_dir_arc.clone();
      let misc_dir_arc_clone = misc_dir_arc.clone();

      let handle = std::thread::spawn(move || {
        // Checks if the file is a directory and if not then it runs the rest
        let is_file_dir: &bool = &file.as_ref().unwrap().file_type().unwrap().is_dir();

        if !is_file_dir {
          // Gets the file's name
          let file_name: String = String::from(
            file.as_ref()
              .unwrap()
              .file_name()
              .into_string()
              .unwrap()
              .trim_start_matches("/"),
          ); // File name
          
          // Gets the file's type
          let file_type: String =
            file_name.clone().split('.').last().unwrap().to_string();

          // Gets how long ago the file was modified
          let file_last_modification = 
            std::time::SystemTime::now().duration_since(
              file.as_ref().unwrap().metadata().unwrap().modified().unwrap() // Gets the last modificaton date of the file
            ).unwrap().as_secs();
            
          // Bool that indicates if the file was already move - if not then it moves it to the misc
          let mut is_moved: bool = false;

          // Goes through every sorting directory
          for i in 0..sr_dirs_arc_clone.len() {
            let current_sr_dir = &sr_dirs_arc_clone[i];

            if is_priority_name_sorting {
              // Sorting by name
              for sr_dir_regex in current_sr_dir
                .sorting
                .by_name
                .trim_start_matches("\"")
                .trim_end_matches("\"")
                .split(' ')
              {
                // For some reason when you split the line by spaces it also has some empty cells
                // so this is an easy fix to this
                if sr_dir_regex == "" { continue; }

                if lib_files::find_pattern(
                  String::from(sr_dir_regex),
                  file_name.clone(),
                ) {
                  let full_file_path =
                    format!("{}/{}", current_dir_arc_clone, file_name);
                  let full_sr_dir_path = format!(
                    "{}/{}",
                    archive_path_arc_clone, current_sr_dir.dir_name
                  );

                  lib_files::move_file(full_file_path, full_sr_dir_path);
                  is_moved = true;

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting diectory {}", file_name, current_sr_dir.dir_name))
                  }
                }
              }

              // Sorting by file's format
              for sr_dir_format in current_sr_dir.sorting.by_format.split(" ") {
                if sr_dir_format == "" { continue; }

                if sr_dir_format == file_type {
                  let full_file_path =
                    format!("{}/{}", current_dir_arc_clone, file_name);

                  let full_sr_dir_path = format!(
                    "{}/{}",
                    archive_path_arc_clone, current_sr_dir.dir_name
                  );

                  lib_files::move_file(full_file_path, full_sr_dir_path);
                  is_moved = true;

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting diectory {}", file_name, current_sr_dir.dir_name))
                  } // Output mode
                } // If the file's type is in the sorting directory
              } // Sorting by file type

              // Sorting by last modification date
              for sr_dir_modification in current_sr_dir.sorting.by_date.split(" ") {
                if sr_dir_modification == "" || sr_dir_modification == "modification" { continue; }

                let full_file_path =
                  format!("{}/{}", current_dir_arc_clone, file_name);

                let full_sr_dir_path = format!(
                  "{}/{}",
                  archive_path_arc_clone, current_sr_dir.dir_name
                );

                let mut is_smaller_than : bool = false;

                let sr_dir_modification = 
                  sr_dir_modification
                  .trim_start_matches("\"").trim_end_matches("\"");

                // Checks what kind of comparision the user wants
                if sr_dir_modification.chars().next().unwrap() == '<' {
                  is_smaller_than = true;
                }

                let sr_dir_modification = 
                  sr_dir_modification
                  .trim_start_matches("<").trim_start_matches(">")
                  .split(":");

                let mut duration_from_config : u64 = 0;
              
                let mut current_time_var : u8 = 0;
                for time_var in sr_dir_modification {
                  match current_time_var {
                    // Hours
                    0 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()*3600).as_secs(),

                    // Minutes
                    1 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()*60).as_secs(),

                    // Seconds
                    2 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()).as_secs(),

                    // Don't wory about it - if there's more than three time parts specified then there will be an error
                    // therefore this line is just a place holder so the compiler won't give an error
                    _ => { break; }
                  }
                  current_time_var += 1;
                }
                drop(current_time_var);

                if is_smaller_than && file_last_modification < duration_from_config {
                  lib_files::move_file(full_file_path, full_sr_dir_path);

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting directory \"{}\"", file_name, current_sr_dir.dir_name));
                  }
                }// Modification earlier
                else if !is_smaller_than && file_last_modification > duration_from_config {
                  lib_files::move_file(full_file_path, full_sr_dir_path);

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting directory \"{}\"", file_name, current_sr_dir.dir_name));
                  }
                } // Modification later
              } // Sorting by modification date
            } // Name sorting priority

            // Sorting by date priority
            else if is_priority_date_sorting {

              // Sorting by last modification date
              for sr_dir_modification in current_sr_dir.sorting.by_date.split(" ") {
                if sr_dir_modification == "" || sr_dir_modification == "modification" { continue; }

                let full_file_path =
                  format!("{}/{}", current_dir_arc_clone, file_name);

                let full_sr_dir_path = format!(
                  "{}/{}",
                  archive_path_arc_clone, current_sr_dir.dir_name
                );

                let mut is_smaller_than : bool = false;

                let sr_dir_modification = 
                  sr_dir_modification
                  .trim_start_matches("\"").trim_end_matches("\"");

                // Checks what kind of comparision the user wants
                if sr_dir_modification.chars().next().unwrap() == '<' {
                  is_smaller_than = true;
                }

                let sr_dir_modification = 
                  sr_dir_modification
                  .trim_start_matches("<").trim_start_matches(">")
                  .split(":");

                let mut duration_from_config : u64 = 0;
              
                let mut current_time_var : u8 = 0;
                for time_var in sr_dir_modification {
                  match current_time_var {
                    // Hours
                    0 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()*3600).as_secs(),

                    // Minutes
                    1 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()*60).as_secs(),

                    // Seconds
                    2 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()).as_secs(),

                    // Don't wory about it - if there's more than three time parts specified then there will be an error
                    // therefore this line is just a place holder so the compiler won't give an error
                    _ => { break; }
                  }
                  current_time_var += 1;
                }
                drop(current_time_var);

                // Moves file if it has been last modified earlier than the specified last modificaton date
                if is_smaller_than && file_last_modification < duration_from_config {
                  lib_files::move_file(full_file_path, full_sr_dir_path);

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting directory \"{}\"", file_name, current_sr_dir.dir_name));
                  }
                } // Modification earlier

                // Moves file if it has been last modified later than the specified last modificaton date
                else if !is_smaller_than && file_last_modification > duration_from_config {
                  lib_files::move_file(full_file_path, full_sr_dir_path);

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting directory \"{}\"", file_name, current_sr_dir.dir_name));
                  }
                } // Modification later
              } // Sorting by date
              
              // Sorting by format
              for sr_dir_format in current_sr_dir.sorting.by_format.split(" ") {
                if sr_dir_format == "" { continue; }

                if sr_dir_format == file_type {
                  let full_file_path =
                    format!("{}/{}", current_dir_arc_clone, file_name);

                  let full_sr_dir_path = format!(
                    "{}/{}",
                    archive_path_arc_clone, current_sr_dir.dir_name
                  );

                  lib_files::move_file(full_file_path, full_sr_dir_path);
                  is_moved = true;

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting diectory {}", file_name, current_sr_dir.dir_name))
                  }
                } // Moves the file if the format is the same as file's
              } // Sorting by format

              // Sorting by name
              for sr_dir_regex in current_sr_dir
                .sorting
                .by_name
                .trim_start_matches("\"")
                .trim_end_matches("\"")
                .split(' ')
              {
                // For some reason when you split the line by spaces it also has some empty cells
                // so this is an easy fix to this
                if sr_dir_regex == "" { continue; }

                if lib_files::find_pattern(
                  String::from(sr_dir_regex),
                  file_name.clone(),
                ) {
                  let full_file_path =
                    format!("{}/{}", current_dir_arc_clone, file_name);
                  let full_sr_dir_path = format!(
                    "{}/{}",
                    archive_path_arc_clone, current_sr_dir.dir_name
                  );

                  lib_files::move_file(full_file_path, full_sr_dir_path);
                  is_moved = true;

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting diectory {}", file_name, current_sr_dir.dir_name))
                  }
                } // Moves the files if the name matches the regex
              } // Sorting by name
            } // Sortinng by date priority

            // If no sorting priority is turned on then it does default sorting by format first
            else {
              // If user has specified to sort only by name this won't sort any file my it's format
              if !is_only_name && !is_only_date {
                for sr_dir_format in current_sr_dir.sorting.by_format.split(" ") {
                  if sr_dir_format == "" {
                    continue;
                  }

                  if sr_dir_format == file_type {
                    let full_file_path =
                      format!("{}/{}", current_dir_arc_clone, file_name);

                    let full_sr_dir_path = format!(
                      "{}/{}",
                      archive_path_arc_clone, current_sr_dir.dir_name
                    );

                    lib_files::move_file(full_file_path, full_sr_dir_path);
                    is_moved = true;

                    if is_output_mode {
                      lib_text::info(format!("Moved file \"{}\" to sorting diectory {}", file_name, current_sr_dir.dir_name))
                    }

                  } // If the sorting directory has the type for the current file
                } // For loop going through every sorting directory
              } // If not sorting only by name or date

              // Same there but with format sorting
              if !is_only_format && !is_only_date{
                for sr_dir_regex in current_sr_dir
                  .sorting
                  .by_name
                  .trim_start_matches("\"")
                  .trim_end_matches("\"")
                  .split(' ')
                {
                  if sr_dir_regex == "" {
                    continue;
                  }

                  if lib_files::find_pattern(
                    String::from(sr_dir_regex),
                    file_name.clone(),
                  ) {
                    let full_file_path =
                      format!("{}/{}", current_dir_arc_clone, file_name);
                    let full_sr_dir_path = format!(
                      "{}/{}",
                      archive_path_arc_clone, current_sr_dir.dir_name
                    );

                    lib_files::move_file(full_file_path, full_sr_dir_path);
                    is_moved = true;
                  
                    if is_output_mode {
                      lib_text::info(format!("Moved file \"{}\" to sorting diectory {}", file_name, current_sr_dir.dir_name))
                    } // Output mode
                  } // If name fits the sorting directory regex
                } // For regex in sorting directory
              } // Sorting by name


              for sr_dir_modification in current_sr_dir.sorting.by_date.split(" ") {
                if sr_dir_modification == "" || sr_dir_modification == "modification" { continue; }

                let full_file_path =
                  format!("{}/{}", current_dir_arc_clone, file_name);

                let full_sr_dir_path = format!(
                  "{}/{}",
                  archive_path_arc_clone, current_sr_dir.dir_name
                );

                let mut is_smaller_than : bool = false;

                let sr_dir_modification = 
                  sr_dir_modification
                  .trim_start_matches("\"").trim_end_matches("\"");

                // Checks what kind of comparision the user wants
                if sr_dir_modification.chars().next().unwrap() == '<' {
                  is_smaller_than = true;
                }

                // Parsing the date for further working on it
                let sr_dir_modification = 
                  sr_dir_modification
                  .trim_start_matches("<").trim_start_matches(">")
                  .split(":");

                let mut duration_from_config : u64 = 0;
              
                let mut current_time_var : u8 = 0;
                for time_var in sr_dir_modification {
                  match current_time_var {
                    // Hours
                    0 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()*3600).as_secs(),

                    // Minutes
                    1 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()*60).as_secs(),

                    // Seconds
                    2 => duration_from_config += std::time::Duration::from_secs(time_var.parse::<u64>().unwrap()).as_secs(),

                    // Don't wory about it - if there's more than three time parts specified then there will be an error
                    // therefore this line is just a place holder so the compiler won't give an error
                    _ => { break; }
                  }
                  current_time_var += 1;
                }
                drop(current_time_var);

                // Moves file if it has been last modified earlier than the specified last modificaton date
                if is_smaller_than && file_last_modification < duration_from_config {
                  lib_files::move_file(full_file_path, full_sr_dir_path);

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting directory \"{}\"", file_name, current_sr_dir.dir_name));
                  }
                } // Modification earlier

                // Moves file if it has been last modified later than the specified last modificaton date
                else if !is_smaller_than && file_last_modification > duration_from_config {
                  lib_files::move_file(full_file_path, full_sr_dir_path);

                  if is_output_mode {
                    lib_text::info(format!("Moved file \"{}\" to sorting directory \"{}\"", file_name, current_sr_dir.dir_name));
                  }
                } // Modification later
              } // Sorting by date
            } // Default sorting order if no priority specified
          } // For through every sorting directory

          // If no sorting directory fitted the file then it's moved to the misc directory (is the misc directory isn't off)
          if !is_moved & is_sorting_misc {
            if is_output_mode {
              lib_text::info(format!("No sorting directory found for file \"{}\". Moving it to the misc folder", file_name));
            }

            let full_file_path = format!("{}/{}", current_dir_arc_clone, file_name);

            let full_sr_dir_path = format!(
              "{}/{}",
              archive_path_arc_clone, misc_dir_arc_clone
            );

            lib_files::move_file(full_file_path, full_sr_dir_path);
          } // Moving to the misc directory
        } // If the file isn't a directory
      }); // End of the handle
      handles.push(handle);
    } // For every file in directory

    // Makes the program wait until all the threads are gone
    for handle in handles {
      let _ = handle.join();
    } // Waiting for the handles
  } // For every directory to clean
} // Main function