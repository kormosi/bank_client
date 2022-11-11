use std::collections::HashSet;
use std::io::{self, Write, Read};
use std::os::unix::net::{UnixStream, UnixDatagram};
use std::path::Path;
use std::{str, fs};

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

fn get_response_from_server() {



}


fn send_instruction_to_server(instruction: &str) -> std::io::Result<()> {
    // create socket
    const SOCK_LOCATION: &str = "/tmp/test.sock";
    let mut stream = UnixStream::connect(SOCK_LOCATION)?;

    stream.write_all(instruction.as_bytes())?;

    Ok(())
}

fn create_listener(socket_location: &str) -> io::Result<UnixDatagram> {
    // Create the socket
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

    let listener = create_listener(SOCK_SRC)?;

    let socket = UnixDatagram::unbound()?;

    // print_instructions();
    socket.connect(SOCK_DST).expect("unable to connect to DST socket");

    loop {

        // const SOCK_LOCATION: &str = "/tmp/test.sock";

        // let instruction = get_valid_instruction_from_user()?;
        // if instruction == "q" {
        //     println!("Quitting, bye!");
        //     return Ok(())
        // }

        println!("sending string");
        socket.send("abc".as_bytes())?;
        println!("sent string");

        // Experimental
        let mut response_buffer = vec![0; 4];
        listener.recv(response_buffer.as_mut_slice());
        let str_from_vec = str::from_utf8(&response_buffer).unwrap();
        println!("{}", str_from_vec);


        // sock.connect("/some/sock").expect("Couldn't connect");
        // sock.send(b"omelette au fromage").expect("send_to function failed");

        // send_instruction_to_server(&instruction)?;

        // println!("sending instruction");

        // stream.write_all(instruction.as_bytes())?;

        // let mut response = String::new();
        // stream.read_to_string(&mut response).expect("couldn't read response");
        // println!("{response}");
    }
}
