use std::process::Command; // Command::new("the command").spawn();
use users::get_current_username;

// All the functions used to print warnings, errors and infos
mod lib;

struct SortingDirectory {
	dir_name : String,
	dir_types : String,
}

fn main() {
	// Gets the user's name
	let mut uname = String::from("");
	match get_current_username() {
		Some(temp_uname) => uname = temp_uname.into_string().unwrap_or(String::from("Unknown error")),
		None => println!("The current user does not exist"),
	}

	// Creates a string which is the path to user's home directory
	let home_path : String =  format!("/home/{uname}");
	drop(uname);

	// All the variables set with configs
	let mut archive_path : String = format!("{}/Archive", home_path);
	let mut misc_dir : String = String::from("Misc");
	let mut sorting_directories : Vec<SortingDirectory> = vec![
			SortingDirectory {
				dir_name : String::from("Videos"),
				dir_types : String::from("mp4 webm mkv vob ogv ogg avi mov viv amv m4p m4v mpg mp2 mpeg mpe mpv m2v m4v"),
		    },SortingDirectory {
				dir_name : String::from("Pictures"),
				dir_types : String::from("jpg png gif mng jfif exif tiff bmp ppm pgm pbm pnm webp heif avif bat cd5 clip cpt kra mdp pdn psd sai xcf svg svgz bmp"),
		    },SortingDirectory {
			    dir_name : String::from("Archive files"),
			    dir_types : String::from("rar 7z jar a ar iso tar br bz2 gz ls lz4 lzma lzo rz sz xz z apk dmg pea zipx zz"),
		    },SortingDirectory {
			    dir_name : String::from("Documents"),
			    dir_types : String::from("doc pdf md odt xml html sxw htm tex txt gdoc docm epub ott rtf uot fodt docx xhtml xml xls wmf tif tiff odp otp odg fodp uop pptx ppsx potx ppt pps pot pptm odf mml"),
		    },SortingDirectory {
			    dir_name : String::from("Audio files"),
			    dir_types : String::from("mp3 wav ogg aiff oga mogg awb 3gp aac flac"),
		    }
    ];

	// All options that can be set up with arguments
	let mut is_silent_mode : bool = false;
	let mut is_output_mode : bool = false;
	let mut is_default_mode : bool = false;
	let mut is_sorting_misc : bool = true;
	
	// Path where to sort the files from
	let mut dirs_to_clear : Vec<String> = vec![];

	// Gets all the arguments
	let mut args : Vec<String> = std::env::args().collect();
	args.remove(0);


	// Argument handling
	for arg in args {
		// gets first two letters of the argument in order to check if it's a long or short argument
		let arg_start : &str = &arg[..2];

		// Checks if the argument is long
		if arg_start == "--" {
			// Pattern matching for long arguments
			let arg_to_match = &arg.trim_start_matches("--");
			match *arg_to_match {
				"silent" => is_silent_mode = true,
				"output" => is_output_mode = true,
				"default" => is_default_mode = true,
				"no-misc" => is_sorting_misc = false,
				"help" => { lib::help_panel(); return; },
				"version" => { lib::version_panel(); return; }
				_ => { if !is_silent_mode { lib::generic_error(format!("Unknown flag: \"{}\"", *arg_to_match)); } return; }
			}
		}
		else if arg.chars().next().unwrap() == '-' {
			// Pattern matching for short arguments
			let split_args = arg.trim_start_matches('-').chars();
			for one_arg in split_args {
				match one_arg {
					's' => is_silent_mode = true,
					'o' => is_output_mode = true,
					'd' => is_default_mode = true,
					'm' => is_sorting_misc = false,
					'h' => { lib::help_panel(); return; },
					'v' => { lib::version_panel(); return; }
					_ =>  { if !is_silent_mode { lib::generic_error(format!("Unknown flag: \"{}\"", one_arg)); } return; }
				}
			}
		}
		else {
			dirs_to_clear.push(arg);
		}
	}

	// Cancels out the output mode if silent mode is selected
	if is_silent_mode { is_output_mode = false; };

	if !is_default_mode {
		let conf_txt = std::fs::read_to_string(format!("{}/.config/clndir/config.conf", &home_path)).unwrap_or(String::from(""));
		if conf_txt != *"" {
			let conf_lines = conf_txt.split('\n');
			let mut directories_declaration : bool = false;
			let mut did_clean : bool = false;
			let mut current_line : u16 = 0;
			for line in conf_lines {
				current_line += 1;
				if line.len() != 0 { // Checks if the line isn't empty
					// Skips the line on comment
					if line.chars().next().unwrap() == '#' {
						continue;
					}

					// Sets the direcotires declaration mode
					if line == "[Directories]" { 
						directories_declaration = true;
						continue;
					}

					let parts : Vec<&str> = line.split('=').collect();
					let param = parts[0].trim();
					let arg = parts[1].trim().replace('"', "").replace('\'', "");
					if arg == "" {
						if !is_silent_mode {
							lib::config_error(current_line, format!("No value assigned to the parameter: \"{}\"", param));
						}
						return;
					}

					if directories_declaration {
						// Checks if it has marked that the default sorting directories were cleared so it
						// doesn't do that again
						if !did_clean {
							sorting_directories.clear();
							did_clean = true;
						}	
						
						// Creates the misc directory if the argument is *
						if arg == "*" {
							misc_dir = String::from(param);
							continue; // Continues to another loop cycle so it doesn't add misc dir to others
						}

						let temp_sorting_dir = SortingDirectory {
							dir_name : String::from(param),
							dir_types : arg,
						};
						sorting_directories.push(temp_sorting_dir);
					}
					else {
						// Sets the custom archive path
						match param {
							"archive_path" => archive_path = arg,
							"misc" => if arg == "false" { is_sorting_misc = false; },
							_ => {if !is_silent_mode { lib::config_error(current_line, format!("Unknown parameter: \"{}\"", param)); } return; },
						};
					}
				}
			}
		}
		else if is_output_mode { // In output mode prints a warning that there's not config file
			lib::generic_warning(String::from("No configs detected in ~/.config/clndir/"));
		}
		drop(conf_txt);
	}
	else if is_output_mode {
		lib::info(String::from("Default mode is on so configs weren't readen"));
	}
		
	// This whole thing creates the archive directory
	let mut current_dir : String = String::from('/');

	for dir in archive_path.trim_matches('/').split('/') {
		// If the searched directory has been found in current directory it stops the loop
		if lib::find_dir(&String::from(dir), &current_dir.clone()) { 
			current_dir = format!("{}{}/", current_dir, dir);
			continue; 
		}

		// Appends another directory anyway to the current directory
		current_dir = format!("{}{}/", current_dir, dir);

		if is_output_mode { lib::info(format!("Directory \"{}\" not found. Creating it in \"{}\"", dir, current_dir)); }

		// If the folder wasn't found in current directory then this creates it
		drop(std::fs::create_dir(&current_dir));
	}
	drop(current_dir);

	let mut is_empty : bool = true;
	for sr_dir in &sorting_directories {
		let mut dir_found : bool = false;
		let sr_dirs_in = std::fs::read_dir(format!("{}/", &archive_path)).unwrap();

		for dir_searching in sr_dirs_in {
			is_empty = false; // If this line isn't executed this means that there aren't any dirs

			// If current checked directory is same name as current checked sorting dir it marks that
			if dir_searching.unwrap().file_name().into_string().unwrap() == sr_dir.dir_name {
				if is_output_mode { lib::info(format!("Directory \"{}\" already found in the archive", sr_dir.dir_name)) }

				dir_found = true;
				break;
			}
		}

		// If the archive directory is empty it creates all the sorting directories
		if is_empty {
			if is_output_mode { lib::info(String::from("The archive directory is empty. Creating all the sorting directories without checking")); }

			for sr_dir in &sorting_directories {
				drop(std::fs::create_dir(&format!("{}/{}", &archive_path, &sr_dir.dir_name)));
			}
			break;
		}

		// If the sorting directoy wasn't found this creates it
		if !dir_found {
			if is_output_mode { lib::info(format!("Directory \"{}\" not found in the archive directory. Creating \"{}\"", sr_dir.dir_name, sr_dir.dir_name)); }

			// Creates the sorting directory that wasn't found in the archive
			drop(std::fs::create_dir(&format!("{}/{}", &archive_path, &sr_dir.dir_name)));
		}
	}

	if dirs_to_clear.len() == 0 { dirs_to_clear.push(String::from(".")); }

	for current_dir in dirs_to_clear {
		// Gets all the files in current directory
		let files_result = std::fs::read_dir(&current_dir);
		let files : std::fs::ReadDir;

		// Either retrieves the file or shows an error
		match files_result {
			Ok(v) => files = v,
			_  => { if !is_silent_mode { 
						lib::generic_error(format!("No directory found at path: {}", current_dir)); 
					} 
					return; }
		}

		// Formats the current path string so it 
		let current_dir = String::from(current_dir.trim_start_matches("/"));

		// Goes through every file and moves them
		for file in files {
			let is_file_dir = file.as_ref().unwrap().file_type().unwrap().is_dir();
			if !is_file_dir {    
				// Gets the file's name
				let file_name = file.as_ref().unwrap().file_name().into_string().unwrap();
				let file_name = file_name.trim_start_matches("/");

				// Gets the file's type
				let file_type = &file_name.split('.').last().unwrap();

				// Bool that indicates if the file was already move - if not then it moves it to the misc
				let mut is_moved : bool = false;


				// Goes through every sorting directory struct and checks if current file belongs to it
				for sr_dir in &sorting_directories {
					if sr_dir.dir_types.find(file_type) != None {
						drop(Command::new("mv")
							// Path to the file to be moved
							.arg(format!("{}/{}", current_dir, file_name))
							// Path to the sorting directory
							.arg(format!("{}/{}/", &archive_path, &sr_dir.dir_name))
							.spawn());

						if is_output_mode { 
							lib::info(format!("Moved file \"{}\" to \"{}{}\"", file_name, archive_path, sr_dir.dir_name));
						}
						is_moved = true;
					}
				}
				if is_sorting_misc & !is_moved {
					drop(Command::new("mv")
						// Path to the file to be moved
						.arg(format!("{}/{}", current_dir, file_name))
						// Path to the misc sorting directory
						.arg(format!("{}/{}/", &archive_path, misc_dir))
						.spawn());
					if is_output_mode {
						lib::info(format!("Moved file \"{}\" to \"{}{}\"", file_name, archive_path, misc_dir));
					}
				}
			}
		}
	}
}
