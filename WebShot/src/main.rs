//! WebShot — A CLI tool to capture website screenshots and download HTML source.
//!
//! This is the main entry point of the application. It parses CLI arguments,
//! initializes the runtime, and dispatches tasks like screenshot capture
//! and HTML source download.
//!
//! Author: Daniel Bretschneider, daniel@bretschneider.cc
//! Version: 1.0
//! Date: 12/11/2025

/// Declare module utils
mod utils;

/// Entry point of the WebShot CLI tool.
fn main() {

    // get cmd args 
    let args = utils::get_command_line_args();

    // print args - run with 'cargo run -- apple banana peach'
    println!("CMD Args: {:?}", args);
}
