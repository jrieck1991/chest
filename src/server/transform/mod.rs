use std::net::TcpStream;
use std::collections::HashMap;
use std::io::{Read, Error};

// read_stream
pub fn read_stream(mut stream: &TcpStream) -> HashMap<Vec<u8>, String> {

    // verify tag
    if !read_tag(&stream) {
        return HashMap::new();
    };

    // parse data and return hashmap
    let h = parse_data(&stream);

    return h;
}

// read_tag will read and verify the first byte in the stream
// expected tag hardcoded to b"0"
fn read_tag(mut stream: &TcpStream) -> bool {

    // buffer to read tag
    let mut tag_buf = [0; 1];

    // read tag from tcp stream
    let _n = match stream.read(&mut tag_buf[..]) {
        Ok(n) => println!("{} bytes read from tcp stream", n),
        Err(_e) => return false,
    };

    // match tag
    // 48 == b"0"
    if tag_buf[0] != 48 {
        println!("invalid tag {}", tag_buf[0]);
        return false
    };

    return true
}

// get_len of data, return 0 if err
fn get_len(mut stream: &TcpStream) -> usize {

    // 8 byte buffer
    let mut len_buf = [0; 8];

    // read stream into buffer
    let _n = match stream.read(&mut len_buf[..]) {
        Ok(_n) => println!("read len success"),
        Err(_e) => return 0,
    };

    // convert bytes to u32
    let data_len = usize::from_be_bytes(len_buf);

    return data_len;
}

// parse_data returns a hashmap parsed from the stream
fn parse_data(mut stream: &TcpStream) -> HashMap<Vec<u8>, String> {

    // init hashmap
    let mut h = HashMap::new();

    loop {

        // get len for key data
        let key_len = get_len(&stream);

        // buffer to read data of length given by len_buf
        let mut key_buf = vec![0; key_len];

        // read data length from tcp stream
        let _n = match stream.read(&mut key_buf) {
            Ok(n) => println!("{} bytes read from tcp stream", n),
            Err(_e) => break,
        };

        h.insert(key_buf, String::from("test_val"));
    }
    
    return h;
}

mod tests {

}