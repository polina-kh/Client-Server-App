use std::net::TcpListener;
use std::thread;
use std::io::{ErrorKind, Read, Write};
use std::sync::mpsc;

const LOCAL: &str = "127.0.0.1:8888";
const MSG_SIZE: usize = 64;

fn main() {
    let server_listener = TcpListener::bind(LOCAL).unwrap();
    server_listener.set_nonblocking(true).unwrap();

    println!("Connection established!");

    let mut clients  = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server_listener.accept(){
            println!("Client {} connected", addr.port());

            let tx = tx.clone();
            clients.push(socket.try_clone().unwrap());

            thread::spawn(move || loop{
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).unwrap();
                        println!("Client {}: {:?}", addr.port(), msg);
                        tx.send(msg).unwrap();
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("Closing connection with Client {}", addr.port());
                            break;
                        }
                }
                sleep();
            });   
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        sleep();   
    }
    
}

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100))
}
