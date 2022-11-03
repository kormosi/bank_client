use std::collections::HashSet;
use std::io::{self, Write};
use std::os::unix::net::UnixStream;

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

fn send_instruction_to_server(instruction: &str) -> std::io::Result<()> {
    const SOCK_LOCATION: &str = "/tmp/test.sock";
    UnixStream::connect(SOCK_LOCATION)?.write_all(instruction.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    print_instructions();

    loop {
        let instruction = get_valid_instruction_from_user()?;

        send_instruction_to_server(&instruction)?;
    }
}
