//----------------------------------------------------//
//--Command line utilities using rust-----------------//
//----------------------------------------------------//

use std::collections::VecDeque;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{ File, OpenOptions };
use std::{ fs, io };
use std::io::{ BufRead, Error as OtherError, ErrorKind, Write };
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    // Getting the arguments from the command line
    let args: Vec<String> = env::args().collect();

    // If the length or arguments is less than 3 the program returns an error
    if args.len() < 3 {
        return Err(Box::new(OtherError::new(ErrorKind::Other, "Missing arguments")));
    }

    let tool = &args[1];

    // Match statement to bind the command with the correspondant function
    match tool.as_str() {
        "echo" => {
            echo(&args[2]);
        }
        "cat" => {
            cat(&args[2..]);
        }
        "ls" => {
            ls(&args[2]);
        }
        "find" => {
            find(&args[2], &args[3..]);
        }
        "grep" => {
            if args.len() >= 5 && args[4] == "-i" {
                grep(&args[3], &args[2], true);
            } else {
                grep(&args[3], &args[2], false);
            }
        }
        _ => {
            println!("Unknown tool: {}", tool);
        }
    }

    Ok(())
}

//-----Echo tool--------//
fn echo(line: &str) {
    println!("{}", line);
}

//-----Cat tool---------//
fn cat(files: &[String]) {
    if files.len() == 1 {
        if let Err(err) = read_and_print(&files[0]) {
            eprintln!("Error reading file: {}", err);
        }
    } else {
        let file_name = &files[files.len() - 1]; // Getting the final filename

        if let Err(err) = create_and_append_to_file(file_name, &files[..files.len() - 1]) {
            eprintln!("Error creating or appending to file: {}", err);
        }
    }
}

//-----ls tool---------//
fn ls(path: &str) {
    if let Err(err) = list_directory(path) {
        eprint!("Error listing files in the given directory: {}", err);
    }
}

//-----find tool---------//

fn find(dir: &str, options: &[String]) {
    let start = dir;
    let mut filename = String::new();
    let mut _type = String::new();
    let default_value = String::new();
    for (i, option) in options.iter().enumerate() {
        // println!("{option}");
        match option.as_str() {
            "-type" => {
                _type = options
                    .get(i + 1)
                    .unwrap_or(&default_value)
                    .clone();
            }
            "-name" => {
                filename = options
                    .get(i + 1)
                    .unwrap_or(&default_value)
                    .clone();
            }
            _ => (),
        }
    }
    if filename.is_empty() {
        eprintln!("Cannot find a file without it's name");
        return;
    }
    let paths = match _type.as_str() {
        "f" => find_file_by_name(&filename, &OsStr::new(start)),
        "d" => find_directory_by_name(&filename, &OsStr::new(start)),
        _ => find_file_by_name(&filename, &OsStr::new(start)),
    };
    match paths {
        Ok(paths) => {
            for path in paths {
                if let Some(p) = path.to_str() {
                    println!("{}", p);
                }
            }
        }
        Err(err) => {
            eprintln!("Error during finding the {}: {}", _type, err);
        }
    }
}

//-----grep tool---------//

fn grep(filename: &str, expression: &str, case_insensitive: bool) {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprint!("Error opening the file: {}", err);
            return;
        }
    };
    let reader = io::BufReader::new(file);
    let mut line_number = 0;
    let mut found = false;
    for line in reader.lines() {
        line_number += 1;
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprint!("Error reading the line: {}", err);
                return;
            }
        };
        let line_to_compare = if case_insensitive { line.to_lowercase() } else { line.clone() };
        let expression_to_compare = if case_insensitive {
            expression.to_lowercase()
        } else {
            expression.to_string()
        };
        if line_to_compare.contains(&expression_to_compare) {
            found = true;
            println!("Line {}: {}", line_number, line);
        }
    }
    if !found {
        println!("Not found!");
    }
}

// Reading file content and printing it

fn read_and_print(path: &str) -> Result<(), Box<dyn Error>> {
    let string_content = fs::read_to_string(path)?;
    println!("{}", string_content);
    Ok(())
}

// Creating and appending to a file

fn create_and_append_to_file(file_name: &str, files: &[String]) -> Result<(), io::Error> {
    let mut data_file = OpenOptions::new().append(true).open(file_name)?;

    for file in files {
        let content = fs::read_to_string(file)?;
        data_file.write_all(content.as_bytes())?;
        data_file.write_all(b"\n")?;
    }
    Ok(())
}

// Listing the files in a directory

fn list_directory(path: &str) -> Result<(), OtherError> {
    let directory = fs::read_dir(path)?;
    for file in directory {
        println!("{}", file?.path().display());
    }
    Ok(())
}

// Find a file by it's name

fn find_file_by_name(query: &str, start: &OsStr) -> io::Result<Vec<PathBuf>> {
    let start = PathBuf::from(start);
    let mut dirs = VecDeque::from(vec![start]);
    let mut result = Vec::new();
    while let Some(dir) = dirs.pop_front() {
        for entry in dir.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                dirs.push_back(path.clone());
            }
            if let Some(name) = path.file_name() {
                if query.is_empty() || query == name {
                    result.push(path.clone());
                }
            }
        }
    }
    if result.is_empty() {
        Err(io::Error::new(io::ErrorKind::NotFound, format!("File '{}' not found", query)))
    } else {
        Ok(result)
    }
}

// Find directory

fn find_directory_by_name(query: &str, start: &OsStr) -> io::Result<Vec<PathBuf>> {
    let start = PathBuf::from(start);
    let mut dirs = VecDeque::from(vec![start]);
    let mut result = Vec::new();
    while let Some(dir) = dirs.pop_front() {
        for entry in dir.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if query.is_empty() || query == name {
                        result.push(path.clone());
                    }
                }
                dirs.push_back(path.clone());
            }
        }
    }
    if result.is_empty() {
        Err(io::Error::new(io::ErrorKind::NotFound, format!("Directory '{}' not found", query)))
    } else {
        Ok(result)
    }
}
