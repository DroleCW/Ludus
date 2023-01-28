use super::connection_resource::ConnectionRes;
use std::io::Write;

pub fn send(connection: &mut ConnectionRes, msg: &[u8]) {
    match &mut connection.0 {
        Some(stream) => {
            println!("{:?}", msg);
            let a = stream.write(msg).unwrap();
            println!("{}", a);
        }
        None => {
            println!("Failed to connect");
        }
    }
}
