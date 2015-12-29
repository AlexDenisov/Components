// Make-related variables are capitalized:
// This is not Rust-way, but makes code readable where we interface with Make
#![allow(non_snake_case)]

static COMPONENTS_VERSION: &'static str = "0.3.0";
static COMPONENTS_DATE:    &'static str = "2015-12-31";

use std::process;
use std::process::Command;
use std::process::Stdio;
use std::env;
use std::fs;
use std::fs::File;
use std::os::unix::prelude::*;

fn main() {
    let command: String;
    let explain_mode: bool;
    let debug_mode: bool = env::vars().any(|k| k.0 == "DEBUG" );
   
    if debug_mode { 
        println!("Debug mode is enabled");
    }
 
    let mut args = env::args();

    if args.len() == 1 {
        println!("Components {} ({})\n\n{}", COMPONENTS_VERSION, COMPONENTS_DATE, COMPONENTS_USAGE_INSTRUCTIONS);

        process::exit(0);
    }

    args.next(); // skipping arg[0]

    let explain_or_command = args.next().unwrap();
    if explain_or_command == "explain" {
         command = args.next().unwrap();
         explain_mode = true;
    } else {
         command = explain_or_command;
         explain_mode = false;
    }

    if debug_mode {
         println!("command is: {}", &command);
    }

    let available_commands = ["install", "purge", "clean", "uninstall", "update"];
    
    let index = available_commands.iter().position(|&r| r == command);
    if index.is_some() == false {
        println!("invalid command: {}", command);
        process::exit(1);
    }

    let current_dir = env::current_dir().unwrap();
    let home_dir    = env::home_dir().unwrap();

    if debug_mode {
        println!("The current directory is: {}", current_dir.display());
        println!("The home directory is: {}", home_dir.display());
    }

    // Local
    let COMPONENTS_TEMP_PATH: &'static str = "/tmp/Components";

    // Global (we'll pass them to Make)
    let COMPONENTS_BUILD_CACHE_PATH: std::path::PathBuf;
    if let Ok(value) = std::env::var("COMPONENTS_BUILD_CACHE_PATH") {
	COMPONENTS_BUILD_CACHE_PATH = std::path::PathBuf::from(value);
    } else {
	COMPONENTS_BUILD_CACHE_PATH = home_dir.join("Library/Caches/Components");
    }

    let COMPONENTS_INSTALL_PATH: std::path::PathBuf;
    if let Ok(value) = std::env::var("COMPONENTS_INSTALL_PATH") {
	COMPONENTS_INSTALL_PATH = std::path::PathBuf::from(value);
    } else {
	COMPONENTS_INSTALL_PATH = current_dir.join("Components");
    }
    
    let COMPONENTS_MAKE_PATH: std::path::PathBuf;
    if let Ok(value) = std::env::var("COMPONENTS_MAKE_PATH") {
        COMPONENTS_MAKE_PATH  = std::path::PathBuf::from(value);
    } else {
        COMPONENTS_MAKE_PATH  = current_dir.join("Components.make");
    }

    if debug_mode {
        println!("COMPONENTS_MAKE_PATH: {}", COMPONENTS_MAKE_PATH.display());
        println!("COMPONENTS_INSTALL_PATH: {}", COMPONENTS_INSTALL_PATH.display());
        println!("COMPONENTS_BUILD_CACHE_PATH: {}", COMPONENTS_BUILD_CACHE_PATH.display());
    }

    if COMPONENTS_MAKE_PATH.exists() == false {
        println!("Directory COMPONENTS_MAKE_PATH does not exist: {:?}", COMPONENTS_MAKE_PATH);
        process::exit(1);
    }

    fs::create_dir_all(&COMPONENTS_BUILD_CACHE_PATH).unwrap_or_else(|_| { 
        println!("failed to create dir COMPONENTS_BUILD_CACHE_PATH: {:?}", COMPONENTS_BUILD_CACHE_PATH);
        process::exit(1);
    });
    
    fs::create_dir_all(&COMPONENTS_INSTALL_PATH).unwrap_or_else(|_| { 
        println!("failed to create dir COMPONENTS_INSTALL_PATH: {:?}", COMPONENTS_INSTALL_PATH);
        process::exit(1);
    });

    fs::create_dir_all(&COMPONENTS_TEMP_PATH).unwrap_or_else(|_| { 
        println!("failed to create dir COMPONENTS_TEMP_PATH: {:?}", COMPONENTS_TEMP_PATH);
        process::exit(1);
    });

    let mut makeflags: Vec<&str> = vec!["--warn-undefined-variables"];

    if explain_mode {
        makeflags.push("-n");
    }

    if debug_mode {
        makeflags.push("-r");
        makeflags.push("-d");
        println!("Makeflags: {:?}", makeflags);
    } 

    let components: Vec<String> = args.collect();

    let component_paths: Vec<std::path::PathBuf>;
 
    if components.len() > 0 {
        if debug_mode {
            println!("Components: {:?}", components);
        }

        let mut should_break = false;

        let existing_components = {
            let existing_component_paths_iterator: fs::ReadDir = fs::read_dir(&COMPONENTS_MAKE_PATH).unwrap();
        
            existing_component_paths_iterator.map(|entry| {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                entry_path.file_stem().unwrap().to_str().unwrap().to_string()
            }).collect::<Vec<String>>()
        };

        for component in &components {
            let component_exists = existing_components.iter().any(|existing_component| existing_component == component);

            if component_exists == false {
                println!("Could not find component: {}", component);
                should_break = true;
            }
        }

        if should_break {
            process::exit(1);
        }

        let selected_component_paths_iterator = {
            let existing_component_paths_iterator: fs::ReadDir = fs::read_dir(&COMPONENTS_MAKE_PATH).unwrap();

            existing_component_paths_iterator.filter(|entry| {
                let entry = entry.as_ref().unwrap();
                let entry_path = entry.path();
                let component_name = entry_path.file_stem().unwrap().to_str().unwrap();
                components.iter().any(|c| c == component_name) 
            })
        };

        component_paths = selected_component_paths_iterator.map(|entry| {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            entry_path
        }).collect::<Vec<std::path::PathBuf>>();
    } else {
        let existing_component_paths_iterator: fs::ReadDir = fs::read_dir(&COMPONENTS_MAKE_PATH).unwrap();

        component_paths = existing_component_paths_iterator.map(|entry| {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            entry_path
        }).collect::<Vec<std::path::PathBuf>>();
    }
    
    let mut success = true;
    for path in component_paths {
        let path_is_valid_component_make = path.is_file() && path.extension().is_some() && path.extension().unwrap() == "make";

        if path_is_valid_component_make == false {
            println!("COMPONENTS_MAKE_PATH directory contains non-builable artefacts (must only consist of *.make)");
            process::exit(1);
        } 

        unsafe {
            let component_name = path.file_stem().unwrap().to_str().unwrap();

            let logpath = format!("/tmp/Components/{}.log", component_name);

            let logfile = File::create(&logpath).unwrap();

            let fd = logfile.as_raw_fd();

            let stdout = if debug_mode { Stdio::inherit() } else { Stdio::from_raw_fd(fd) };
            let stderr = if debug_mode { Stdio::inherit() } else { Stdio::from_raw_fd(fd) };
          
            print!("[{}] {}:", command, component_name);

            let status = Command::new("make")
                .arg(&command)
                .args(&makeflags)
                .arg("-f")
                .arg(path.to_str().unwrap())
                .env("COMPONENTS_INSTALL_PATH", COMPONENTS_INSTALL_PATH.as_os_str())
                .env("COMPONENTS_MAKE_PATH", COMPONENTS_MAKE_PATH.as_os_str())
                .stdout(stdout)
                .stderr(stderr)
                .status().unwrap_or_else(|e| {
                    panic!("failed to execute process: {}", e)
                });

            if status.success() {
                print!(" done.\n");
            } else {
                print!(" failed. See log for details:\n{}\n", &logpath);
                success = false;
            }
        }
    }

    if success {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

static COMPONENTS_USAGE_INSTRUCTIONS: &'static str = "\
Usage:
  components [explain] command [components]

  command:
    install   - downloads, builds (if needed) and installs component 
                into a vendor directory (COMPONENTS_INSTALL_PATH)
    uninstall - removes component from a vendor directory (COMPONENTS_INSTALL_PATH)
    clean     - removes downloaded files and built artefacts from 
                cache directory (COMPONENTS_BUILD_CACHE_PATH)
    purge     - cleans and uninstalls component
    update    - uninstalls current and installs new version of a component

  explain:
    print the commands that would be executed, but do not execute them

  components:
    one or more component name, enumerates all components in the components 
    directory (COMPONENTS_MAKE_PATH) if the parameter is omitted

Directories:
  COMPONENTS_MAKE_PATH         - directory contains all components used in the current project
                               $COMPONENTS_MAKE_PATH
  COMPONENTS_INSTALL_PATH      - directory contains all installed components for the current project
                               $COMPONENTS_INSTALL_PATH
  COMPONENTS_BUILD_CACHE_PATH  - stores zip/tarballs, built artefacts, or source code of used components
                               $COMPONENTS_BUILD_CACHE_PATH

Debugging:
  Set environment variable 'DEBUG' to 'YES' to enable debugging output.
  It will run make with additional parameters '-r -d' and will send output to 'STDOUT'.

  From 'man make':
    -d  Print debugging information in addition to normal processing.
    -r  Eliminate use of the built−in implicit rules.

Examples:
  components install                # installs every component from the components directory (COMPONENTS_MAKE_PATH)
  components purge Cedar            # cleans and uninstalls Cedar library
  components update BloodMagic      # uninstalls current and installs new version of BloodMagic library
  components clean BloodMagic Cedar # cleans up Cedar' and BloodMagic' artefacts
  components explain install        # prints commands that would be executed to install each component
  DEBUG=YES components install      # prints additional information to 'STDOUT'\
";

