use std::env;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use french_press::{compress, decompress};

fn print_usage(program_name: &str) {
    eprintln!("Usage: {program_name} <-c|--compress|-d|--decompress> <input_file> [output_file]");
    eprintln!("\nFlags:");
    eprintln!("  -c, --compress      Compress the input file");
    eprintln!("  -d, --decompress    Decompress the input file");
}

fn determine_output_path(input_path: &Path, is_compress: bool) -> PathBuf {
    if is_compress {
        let mut out = input_path.as_os_str().to_os_string();
        out.push(".fp");
        PathBuf::from(out)
    } else if input_path.extension().and_then(|s| s.to_str()) == Some("fp") {
        input_path.with_extension("")
    } else {
        let mut out = input_path.as_os_str().to_os_string();
        out.push(".out");
        PathBuf::from(out)
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let program_name = args.first().map(|s| s.as_str()).unwrap_or("french_press");

    if args.len() < 3 {
        print_usage(program_name);
        return Err("Missing required arguments".to_string());
    }

    let mode = &args[1];
    let input_path = Path::new(&args[2]);

    match mode.as_str() {
        "-c" | "--compress" => {
            let input_file = File::open(input_path)
                .map_err(|err| format!("Failed to open '{}': {err}", input_path.display()))?;
            let input_size = input_file
                .metadata()
                .map_err(|err| {
                    format!(
                        "Failed to get metadata for '{}': {err}",
                        input_path.display()
                    )
                })?
                .len();
            let mut input_reader = BufReader::new(input_file);

            let out_path = if args.len() >= 4 {
                PathBuf::from(&args[3])
            } else {
                determine_output_path(input_path, true)
            };

            let out_file = File::create(&out_path)
                .map_err(|err| format!("Failed to create '{}': {err}", out_path.display()))?;
            let mut output_writer = BufWriter::new(out_file);

            compress(&mut input_reader, &mut output_writer)
                .map_err(|e| format!("compression failed: {e}"))?;

            output_writer
                .flush()
                .map_err(|e| format!("failed to flush '{}': {e}", out_path.display()))?;

            let output_size = fs::metadata(&out_path).map(|m| m.len()).unwrap_or(0);

            println!(
                "Successfully compressed '{}' -> '{}' ({} -> {} bytes)",
                input_path.display(),
                out_path.display(),
                input_size,
                output_size
            );
            Ok(())
        }
        "-d" | "--decompress" => {
            let input_file = File::open(input_path)
                .map_err(|err| format!("Failed to open '{}': {err}", input_path.display()))?;
            let input_size = input_file
                .metadata()
                .map_err(|err| {
                    format!(
                        "Failed to get metadata for '{}': {err}",
                        input_path.display()
                    )
                })?
                .len();
            let mut input_reader = BufReader::new(input_file);

            let out_path = if args.len() >= 4 {
                PathBuf::from(&args[3])
            } else {
                determine_output_path(input_path, false)
            };

            let out_file = File::create(&out_path)
                .map_err(|err| format!("Failed to create '{}': {err}", out_path.display()))?;
            let mut output_writer = BufWriter::new(out_file);

            decompress(&mut input_reader, &mut output_writer)
                .map_err(|err| format!("Failed to decompress '{}': {err}", input_path.display()))?;

            output_writer
                .flush()
                .map_err(|e| format!("failed to flush '{}': {e}", out_path.display()))?;

            let output_size = fs::metadata(&out_path).map(|m| m.len()).unwrap_or(0);

            println!(
                "Successfully decompressed '{}' -> '{}' ({} -> {} bytes)",
                input_path.display(),
                out_path.display(),
                input_size,
                output_size
            );
            Ok(())
        }
        _ => {
            print_usage(program_name);
            Err(format!(
                "Unknown flag '{mode}'. Expected '-c', '--compress', '-d', or '--decompress'."
            ))
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
