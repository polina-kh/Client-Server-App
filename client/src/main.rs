use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:8888";
const MSG_SIZE: usize = 64;

fn main() {
    let mut client = TcpStream::connect(LOCAL).unwrap();
    client.set_nonblocking(true).unwrap();

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                //println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Сonnection was severed");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).unwrap();
                println!("Your message sent {:?}", msg);
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        sleep();
    });

    println!("Please write a message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).unwrap();
        let msg = buff.trim().to_string();
        if msg == ":q!" || tx.send(msg).is_err() {break}
    }
    println!("Your connection is closed!");

}

fn sleep() {
    thread::sleep(Duration::from_millis(100))
}
