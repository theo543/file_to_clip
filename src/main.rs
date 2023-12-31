#![windows_subsystem = "windows"]

use std::process::ExitCode;
use std::io::Write;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
fn main() -> ExitCode {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: file_to_clip <path>");
        return ExitCode::FAILURE;
    }

    let content = std::fs::read_to_string(&args[1]).expect("Failed to read file");

    let mut clip = std::process::Command::new("clip")
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn clip");
    let mut clip_in = clip.stdin.take().expect("Failed to get clip stdin");
    clip_in
        .write(content.as_bytes())
        .expect("Failed to write to clip stdin");
    drop(clip_in);
    clip.wait().expect("Failed to wait for clip");

    ExitCode::SUCCESS
}
