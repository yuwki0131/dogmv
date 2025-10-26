use log::info;
use std::env;
use std::path::{Path, PathBuf};

/// Parses command-line arguments and returns the initial file and root directory.
///
/// # Arguments
/// * `args` - Command-line arguments (typically from env::args())
///
/// # Returns
/// * `(Option<PathBuf>, PathBuf)` - (initial file to open, root directory for tree view)
///
/// # Behavior
/// - No arguments: Uses current directory as root, no initial file
/// - File argument: Opens the file, uses parent directory as root
/// - Directory argument: Uses directory as root, no initial file
///
/// # Exits
/// Exits the process with code 1 if:
/// - The specified path doesn't exist
/// - The path is neither a file nor a directory
pub fn parse_arguments(args: &[String]) -> (Option<PathBuf>, PathBuf) {
    if args.len() < 2 {
        // No arguments: use current directory
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        info!("No arguments provided, using current directory: {:?}", current_dir);
        return (None, current_dir);
    }

    let arg_path = Path::new(&args[1]);

    if !arg_path.exists() {
        eprintln!("Error: Path not found: {}", args[1]);
        std::process::exit(1);
    }

    if arg_path.is_file() {
        // File specified: open file and use parent directory as root
        let parent_dir = arg_path.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        info!("File specified: {:?}, root directory: {:?}", arg_path, parent_dir);
        (Some(arg_path.to_path_buf()), parent_dir)
    } else if arg_path.is_dir() {
        // Directory specified: use as root directory, no initial file
        info!("Directory specified: {:?}", arg_path);
        (None, arg_path.to_path_buf())
    } else {
        eprintln!("Error: Invalid path: {}", args[1]);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_no_args() {
        let args = vec!["dogmv".to_string()];
        let (file, root) = parse_arguments(&args);
        assert!(file.is_none());
        assert!(root.is_absolute() || root.as_os_str() == ".");
    }
}
