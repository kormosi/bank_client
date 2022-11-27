use std::collections::{HashSet};
use std::io::{self, Write};
use std::os::unix::net::UnixDatagram;
use std::path::Path;
use std::{fs, str};

use anyhow::Result;
use serde::Serialize;
use serde_json;

fn print_instructions() {
    println!(
        "i: account info
t: perform transaction
?: show instructions
q: quit\n"
    );
}

#[derive(Serialize)]
struct TxInfo {
    from: String,
    to: String,
    amount: u64,
}

fn get_tx_info() -> Result<TxInfo> {
    let from = prompt_and_get_input("from")?;
    let to = prompt_and_get_input("to")?;
    let amount = prompt_and_get_input("amount")?.parse::<u64>()?;

    Ok(TxInfo { from, to, amount })
}

fn prompt_and_get_input(prompt: &str) -> Result<String, io::Error> {
    let mut from = String::new();
    print!("{}: ", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut from)?;
    let from = from.trim();
    Ok(from.to_string())
}

fn get_valid_instruction_from_user() -> Result<String, io::Error> {
    let valid_inputs = HashSet::from(["i", "t", "q", "?"]);

    loop {
        let mut user_input = String::new();
        print!("#: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        if let Some(user_input) = valid_inputs.get(user_input) {
            return Ok(user_input.to_string());
        } else {
            println!("Invalid instruction");
        }
    }
}

fn create_socket(socket_location: &str) -> io::Result<UnixDatagram> {
    let socket_path = Path::new(socket_location);
    if socket_path.exists() {
        fs::remove_file(socket_path)?;
    }

    match UnixDatagram::bind(socket_path) {
        Err(e) => return Err(e),
        Ok(listener) => return Ok(listener),
    };
}

fn main() -> Result<()> {
    const SOCK_SRC: &str = "/tmp/client2server.sock";
    const SOCK_DST: &str = "/tmp/server2client.sock";

    let socket = create_socket(SOCK_SRC)?;

    print_instructions();

    loop {
        let instruction = get_valid_instruction_from_user()?;
        match instruction.as_str() {
            "q" => {
                println!("Quitting, bye!");
                return Ok(());
            }
            "t" => {
                // Get and send tx_info to socket
                let tx_info = get_tx_info().unwrap();
                let tx_info_serialized = serde_json::to_string(&tx_info).unwrap();
                socket.send_to(instruction.as_bytes(), SOCK_DST)?;
                // Receive and verify "handshake"
                let mut response_buffer = vec![0; 3];
                socket.recv(response_buffer.as_mut_slice())?;
                let str_response = str::from_utf8(&response_buffer)?;
                if str_response.trim() == "200" {
                    socket.send_to(tx_info_serialized.as_bytes(), SOCK_DST)?;

                    let mut response_buffer = vec![0; 50];
                    socket.recv(response_buffer.as_mut_slice())?;
                    let str_response = str::from_utf8(&response_buffer)?.trim();
                    println!("{str_response}");
                }
            }
            "i" => {
                socket.send_to(instruction.as_bytes(), SOCK_DST)?;
                let mut response_buffer = vec![0; 500];
                socket.recv(response_buffer.as_mut_slice())?;
                let str_from_vec = str::from_utf8(&response_buffer).unwrap();
                println!("{}", str_from_vec);
            },
            "?" => {
                print_instructions();
            },
            _ => unreachable!()
        }

        // socket.send_to(instruction.as_bytes(), SOCK_DST)?;

        // let mut response_buffer = vec![0; 500];
        // socket.recv(response_buffer.as_mut_slice())?;
        // let str_from_vec = str::from_utf8(&response_buffer).unwrap();
        // println!("{}", str_from_vec);
    }
}
