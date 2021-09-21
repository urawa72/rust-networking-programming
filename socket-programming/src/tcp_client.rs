use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn connet(address: &str) -> Result<(), failure::Error> {
    let mut stream = TcpStream::connect(address)?;
    loop {
        // send input data to socket
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

        // display received data from socket
        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        print!("{}", std::str::from_utf8(&buffer)?);
    }
}
