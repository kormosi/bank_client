use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::os::unix::net::UnixDatagram;
use std::path::Path;
use std::{fs, str};

use anyhow::Result;
use serde_json;
use serde::{Serialize};

fn print_instructions() {
    println!(
        "i: account info
t: perform transaction
q: quit\n"
    );
}

#[derive(Serialize)]
enum TxInfoValue {
    Name(String),
    Amount(u64),
}

fn get_tx_info() -> Result<HashMap<String, TxInfoValue>> {
    let from = prompt_and_get_input("from")?;
    let to = prompt_and_get_input("to")?;
    let amount = prompt_and_get_input("amount")?;
    let amount = amount.parse::<u64>()?;

    let mut tx_info = HashMap::new();
    tx_info.insert("from".to_string(), TxInfoValue::Name(from));
    tx_info.insert("to".to_string(), TxInfoValue::Name(to));
    tx_info.insert("amount".to_string(), TxInfoValue::Amount(amount));

    Ok(tx_info)
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

fn create_listener(socket_location: &str) -> io::Result<UnixDatagram> {
    let socket_path = Path::new(socket_location);
    if socket_path.exists() {
        fs::remove_file(socket_path)?;
    }

    match UnixDatagram::bind(socket_path) {
        Err(e) => return Err(e),
        Ok(listener) => return Ok(listener),
    };
}

fn main() -> Result<(), io::Error> {
    const SOCK_SRC: &str = "/tmp/client2server.sock";
    const SOCK_DST: &str = "/tmp/server2client.sock";

    let socket = create_listener(SOCK_SRC)?;

    print_instructions();

    loop {
        let instruction = get_valid_instruction_from_user()?;
        match instruction.as_str() {
            "q" => {
                println!("Quitting, bye!");
                return Ok(());
            }
            "t" => {
                let tx_info = get_tx_info().unwrap();
                let tx_info_serialized = serde_json::to_string(&tx_info).unwrap();
                println!("{tx_info_serialized}");
            }
            _ => panic!("blabla"),
        }

        socket.send_to(instruction.as_bytes(), SOCK_DST)?;

        let mut response_buffer = vec![0; 500];
        socket.recv(response_buffer.as_mut_slice())?;
        let str_from_vec = str::from_utf8(&response_buffer).unwrap();
        println!("{}", str_from_vec);
    }
}
