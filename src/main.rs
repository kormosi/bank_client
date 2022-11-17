use std::collections::HashSet;
use std::io::{self, Write};
use std::os::unix::net::UnixDatagram;
use std::path::Path;
use std::{fs, str};

fn print_instructions() {
    println!(
        "i: account info
t: perform transaction
q: quit\n"
    );
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
        if instruction == "q" {
            println!("Quitting, bye!");
            return Ok(());
        }

        socket.send_to(instruction.as_bytes(), SOCK_DST)?;

        let mut response_buffer = vec![0; 500];
        socket.recv(response_buffer.as_mut_slice())?;
        let str_from_vec = str::from_utf8(&response_buffer).unwrap();
        println!("{}", str_from_vec);
    }
}
