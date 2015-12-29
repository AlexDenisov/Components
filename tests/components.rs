// Make-related variables are capitalized:
// This is not Rust-way, but makes code readable where we interface with Make
#![allow(non_snake_case)]

extern crate tempdir;

use std::process::Command;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tempdir::*;

static COMPONENT_TRIVIAL_CONTENTS: &'static str = "\
NAME=Component_Trivial
COMPONENTS_INSTALL_PATH ?= ./Components
COMPONENT_INSTALL_PATH ?= $(COMPONENTS_INSTALL_PATH)/$(NAME)

install:
	mkdir -p $(COMPONENT_INSTALL_PATH)
	touch $(COMPONENT_INSTALL_PATH)/Component_Trivial.txt
";

static COMPONENT_FAILING_CONTENTS: &'static str = "\
install:
	exit 1
";

#[test]
fn it_shows_usage_instructions() {
    let current_dir = env::current_dir().unwrap();

    let COMPONENTS_EXEC;
    if let Ok(value) = std::env::var("COMPONENTS_EXEC") {
        COMPONENTS_EXEC = current_dir.join(value);
    } else {
        panic!("Didn't get executable to run");
    }

    let output = Command::new(COMPONENTS_EXEC)
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    let output_string = String::from_utf8_lossy(&output.stdout);
    
    //println!("status: {}", output.status);
    //println!("status: {}", output_string);

    assert!(output.status.success());
    assert!(output_string.contains("Usage:"));
    assert!(output_string.contains("components [explain] command [components]"));
}

#[test]
fn it_fails_and_reports_on_invalid_command() {
    let current_dir = env::current_dir().unwrap();

    let COMPONENTS_EXEC;
    if let Ok(value) = std::env::var("COMPONENTS_EXEC") {
        COMPONENTS_EXEC = current_dir.join(value);
    } else {
        panic!("Didn't get executable to run");
    }

    let output = Command::new(COMPONENTS_EXEC)
        .arg("wrong")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    let output_string = String::from_utf8_lossy(&output.stdout);
    
    //println!("status: {}", output.status);
    //println!("status: {}", output_string);

    assert!(output.status.success() == false);
    assert!(output_string.contains("invalid command: wrong"));
}

#[test]
fn it_installs_component() {
    let current_dir = env::current_dir().unwrap();
    let home_dir    = env::home_dir().unwrap();
    let tmp_dir     = env::temp_dir();
    let stage_dir   = TempDir::new_in(tmp_dir.as_path(), "Components-Test").unwrap();
    
    println!("The current directory is:   {}", current_dir.display());
    println!("The home directory is:      {}", home_dir.display());
    println!("The temporary directory is: {}", tmp_dir.display());

    let components_install_dir     = TempDir::new_in(stage_dir.path(), "Components").unwrap();
    let components_make_dir        = TempDir::new_in(stage_dir.path(), "Components.make").unwrap();
    let components_build_cache_dir = TempDir::new_in(stage_dir.path(), "Cache").unwrap();

    let COMPONENTS_EXEC;
    if let Ok(value) = std::env::var("COMPONENTS_EXEC") {
        COMPONENTS_EXEC = current_dir.join(value);
    } else {
        panic!("Didn't get executable to run");
    }

    println!("COMPONENTS_INSTALL_PATH     = {:?}", components_install_dir.path());
    println!("COMPONENTS_MAKE_PATH        = {:?}", components_make_dir.path());
    println!("COMPONENTS_BUILD_CACHE_PATH = {:?}", components_build_cache_dir.path());
    println!("COMPONENTS_EXEC             = {:?}", COMPONENTS_EXEC);
   
    let component_make_path = components_make_dir.path().join("Component-Trivial.make");

    println!("Component.make: {:?}", component_make_path);

    let mut f = File::create(component_make_path).unwrap();

    f.write_all(COMPONENT_TRIVIAL_CONTENTS.as_bytes());

    let output = Command::new(COMPONENTS_EXEC)
        .arg("install")
        .env("COMPONENTS_INSTALL_PATH", components_install_dir.path().as_os_str())
        .env("COMPONENTS_MAKE_PATH", components_make_dir.path().as_os_str())
        .env("COMPONENTS_BUILD_CACHE_PATH", components_build_cache_dir.path().as_os_str())
        //.env("DEBUG", "YES")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    let output_string = String::from_utf8_lossy(&output.stdout);
    
    //println!("status: {}", output.status);
    //println!("status: {}", output_string);

    let component_file_path = components_install_dir.path().join("Component_Trivial").join("Component_Trivial.txt");
    
    assert!(output.status.success());
    assert!(output_string.contains("[install] Component-Trivial: done."));
    assert!(component_file_path.exists());
    assert!(component_file_path.is_file());

    //std::thread::sleep_ms(1000000);
}

#[test]
fn it_doesnot_install_broken_component_and_reports_error() {
    let current_dir = env::current_dir().unwrap();
    let home_dir    = env::home_dir().unwrap();
    let tmp_dir     = env::temp_dir();
    let stage_dir   = TempDir::new_in(tmp_dir.as_path(), "Components-Test").unwrap();
    
    println!("The current directory is:   {}", current_dir.display());
    println!("The home directory is:      {}", home_dir.display());
    println!("The temporary directory is: {}", tmp_dir.display());

    let components_install_dir     = TempDir::new_in(stage_dir.path(), "Components").unwrap();
    let components_make_dir        = TempDir::new_in(stage_dir.path(), "Components.make").unwrap();
    let components_build_cache_dir = TempDir::new_in(stage_dir.path(), "Cache").unwrap();

    let COMPONENTS_EXEC;
    if let Ok(value) = std::env::var("COMPONENTS_EXEC") {
        COMPONENTS_EXEC = current_dir.join(value);
    } else {
        panic!("Didn't get executable to run");
    }

    println!("COMPONENTS_INSTALL_PATH     = {:?}", components_install_dir.path());
    println!("COMPONENTS_MAKE_PATH        = {:?}", components_make_dir.path());
    println!("COMPONENTS_BUILD_CACHE_PATH = {:?}", components_build_cache_dir.path());
    println!("COMPONENTS_EXEC             = {:?}", COMPONENTS_EXEC);
   
    let component_make_path = components_make_dir.path().join("Component-Failing.make");

    println!("Component.make: {:?}", component_make_path);

    let mut f = File::create(component_make_path).unwrap();

    f.write_all(COMPONENT_FAILING_CONTENTS.as_bytes());

    let output = Command::new(COMPONENTS_EXEC)
        .arg("install")
        .env("COMPONENTS_INSTALL_PATH", components_install_dir.path().as_os_str())
        .env("COMPONENTS_MAKE_PATH", components_make_dir.path().as_os_str())
        .env("COMPONENTS_BUILD_CACHE_PATH", components_build_cache_dir.path().as_os_str())
        //.env("DEBUG", "YES")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    let output_string = String::from_utf8_lossy(&output.stdout);
    
    //println!("status: {}", output.status);
    //println!("status: {}", output_string);

    assert!(output.status.success() == false);
    assert!(output_string.contains("[install] Component-Failing: failed. See log for details:\n/tmp/Components/Component-Failing.log"));
}

#[test]
fn it_refuses_to_operate_on_directory_with_non_make_stuff() {
    let current_dir = env::current_dir().unwrap();
    let home_dir    = env::home_dir().unwrap();
    let tmp_dir     = env::temp_dir();
    let stage_dir   = TempDir::new_in(tmp_dir.as_path(), "Components-Test").unwrap();
    
    println!("The current directory is:   {}", current_dir.display());
    println!("The home directory is:      {}", home_dir.display());
    println!("The temporary directory is: {}", tmp_dir.display());

    let components_install_dir     = TempDir::new_in(stage_dir.path(), "Components").unwrap();
    let components_make_dir        = TempDir::new_in(stage_dir.path(), "Components.make").unwrap();
    let components_build_cache_dir = TempDir::new_in(stage_dir.path(), "Cache").unwrap();

    let COMPONENTS_EXEC;
    if let Ok(value) = std::env::var("COMPONENTS_EXEC") {
        COMPONENTS_EXEC = current_dir.join(value);
    } else {
        panic!("Didn't get executable to run");
    }

    println!("COMPONENTS_INSTALL_PATH     = {:?}", components_install_dir.path());
    println!("COMPONENTS_MAKE_PATH        = {:?}", components_make_dir.path());
    println!("COMPONENTS_BUILD_CACHE_PATH = {:?}", components_build_cache_dir.path());
    println!("COMPONENTS_EXEC             = {:?}", COMPONENTS_EXEC);
   
    let component_make_path = components_make_dir.path().join("NON-COMPONENT-FILE");

    let mut f = File::create(&component_make_path).unwrap();
    f.write_all("BLIP!\n".as_bytes());

    println!("---- {:?}", component_make_path);
    let output = Command::new(COMPONENTS_EXEC)
        .arg("install")
        .env("COMPONENTS_INSTALL_PATH", components_install_dir.path().as_os_str())
        .env("COMPONENTS_MAKE_PATH", components_make_dir.path().as_os_str())
        .env("COMPONENTS_BUILD_CACHE_PATH", components_build_cache_dir.path().as_os_str())
        //.env("DEBUG", "YES")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    let output_string = String::from_utf8_lossy(&output.stdout);
    
    //println!("status: {}", output.status);
    println!("status: {}", output_string);

    assert!(output.status.success() == false);
    assert!(output_string.contains("COMPONENTS_MAKE_PATH directory contains non-builable artefacts (must only consist of *.make)"));
}

#[test]
fn it_stops_when_COMPONENT_MAKE_PATH_directory_does_not_exist() {
    let current_dir = env::current_dir().unwrap();
    let home_dir    = env::home_dir().unwrap();
    let tmp_dir     = env::temp_dir();
    let stage_dir   = TempDir::new_in(tmp_dir.as_path(), "Components-Test").unwrap();
    
    println!("The current directory is:   {}", current_dir.display());
    println!("The home directory is:      {}", home_dir.display());
    println!("The temporary directory is: {}", tmp_dir.display());

    let components_install_dir     = TempDir::new_in(stage_dir.path(), "Components").unwrap();
    let components_build_cache_dir = TempDir::new_in(stage_dir.path(), "Cache").unwrap();

    let components_make_non_existing_path = stage_dir.path().join("Components.make");

    let COMPONENTS_EXEC;
    if let Ok(value) = std::env::var("COMPONENTS_EXEC") {
        COMPONENTS_EXEC = current_dir.join(value);
    } else {
        panic!("Didn't get executable to run");
    }
    
    println!("COMPONENTS_INSTALL_PATH     = {:?}", components_install_dir.path());
    println!("COMPONENTS_BUILD_CACHE_PATH = {:?}", components_build_cache_dir.path());
    println!("COMPONENTS_EXEC             = {:?}", COMPONENTS_EXEC);
   
    let output = Command::new(COMPONENTS_EXEC)
        .arg("install")
        .env("COMPONENTS_INSTALL_PATH", components_install_dir.path().as_os_str())
        .env("COMPONENTS_MAKE_PATH", components_make_non_existing_path.as_os_str())
        .env("COMPONENTS_BUILD_CACHE_PATH", components_build_cache_dir.path().as_os_str())
        //.env("DEBUG", "YES")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    let output_string = String::from_utf8_lossy(&output.stdout);
    
    //println!("status: {}", output_string);

    assert!(output.status.success() == false);
    assert!(output_string.contains("Directory COMPONENTS_MAKE_PATH does not exist:"));
}
