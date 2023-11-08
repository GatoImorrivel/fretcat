use std::env;
use std::fs::{self, File};
use std::io;
use std::path::Path;

fn concatenate_files_in_folder(folder_path: &str, output: &mut File) -> io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let mut file = File::open(&path)?;
            io::copy(&mut file, output)?;
        } else if path.is_dir() {
            concatenate_files_in_folder(&path.to_string_lossy(), output)?;
        }
    }

    Ok(())
}

fn main() {
    // Get the current library path (project root directory).
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory not found");

    // Specify the root folders to search for files.
    let folder_paths = [
        "editor",
        "effects",
        // Add more folder paths as needed.
    ];

    // Construct the full path for the output file based on the manifest directory.
    let output_file_path = Path::new(&manifest_dir).join("fretcat-styles.css");
    println!("{}", output_file_path.display());

    // Open the output file for writing.
    let mut output = File::create(&output_file_path).expect("Failed to create output file");

    // Iterate through the specified folders and concatenate files within them.
    for folder_path in &folder_paths {
        if let Err(err) = concatenate_files_in_folder(folder_path, &mut output) {
            eprintln!("Error while concatenating files in folder {}: {}", folder_path, err);
        }
    }

    println!("Concatenated and merged files from specified folders into '{}'", output_file_path.display());
}