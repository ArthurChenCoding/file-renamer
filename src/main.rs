use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

fn validate_args(args: &[String]) -> Result<(&str, &str, &str), &'static str> {
    if args.len() != 4 {
        return Err("Usage: file_renamer <directory> <search_pattern> <replacement>");
    }
    let directory = &args[1];
    let search_pattern = &args[2];
    let replacement = &args[3];

    if !Path::new(directory).is_dir() {
        return Err("The specified directory does not exist.");
    }

    Ok((directory, search_pattern, replacement))
}

fn rename_files(directory: &str, search_pattern: &str, replacement: &str) -> Result<(), String> {
    let re = match Regex::new(search_pattern) {
        Ok(re) => re,
        Err(e) => return Err(format!("Invalid regex pattern: {}", e)),
    };

    for entry in fs::read_dir(directory).map_err(|e| format!("Error reading directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Error reading entry: {}", e))?;
        let old_path = entry.path();
        let old_filename = entry.file_name().into_string().unwrap();

        if let Some(new_filename) = re.replace_all(&old_filename, replacement).as_ref() {
            if new_filename != old_filename {
                let new_path = Path::new(directory).join(new_filename);
                fs::rename(&old_path, &new_path)
                    .map_err(|e| format!("Error renaming file: {}", e))?;
                println!("Renamed: {} -> {}", old_filename, new_filename);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (directory, search_pattern, replacement) = match validate_args(&args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    match rename_files(directory, search_pattern, replacement) {
        Ok(_) => println!("Files renamed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
