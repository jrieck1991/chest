use serde::{Serialize, Deserialize};

pub struct Transform {
    tag: u8,
}

impl Transform {

    pub fn new(t: u8) -> Transform {
        Transform{
            tag: t,
        }
    }

    pub fn encode(data String) {

        let data_len = data.as_bytes().len();

        // create buffer
        let mut buf: Vec<u8> = Vec::with_capacity(data_len)

        // tag == u32 48
        buf.push(b"0"[0]);

        // TODO: dont iterate
        // add data length bytes
        for l in data_len.to_be_bytes().iter() {
            buf.push(*l);
        }

        // TODO: dont iterate
        // add data
        for d in data.to_be_bytes().iter() {
            buf.push(*d);
        }
    }

    pub fn decode() {

        // buffer to read tag
        let mut tag_buf = [0; 1];

        // read tag from tcp stream
        let _n = match stream.read(&mut tag_buf[..]) {
            Ok(n) => println!("{} bytes read from tcp stream", n),
            Err(e) => println!("error reading from stream: {}", e),
        };

        // match tag
        // 48 == b"0"
        if tag_buf[0] != 48 {
            println!("invalid tag {}", tag_buf[0]);
            return;
        };

        // buffer to read data size in bytes
        let mut len_buf = [0; 8];

        // read data length from tcp stream
        let _n = match stream.read(&mut len_buf[..]) {
            Ok(n) => println!("{} bytes read from tcp stream", n),
            Err(e) => println!("error reading from stream: {}", e),
        };

        // convert bytes to u32
        let data_len = usize::from_be_bytes(len_buf);

        // buffer to read data of length given by len_buf
        let mut data_buf = vec![0; data_len];

        // read data length from tcp stream
        let _n = match stream.read(&mut data_buf) {
            Ok(n) => println!("{} bytes read from tcp stream", n),
            Err(e) => println!("error reading from stream: {}", e),
        };

        // iterate over buffer
        for (_i, x) in data_buf.into_iter().enumerate() {
            println!("data byte: {}", x)
        }
    }
}

mod tests {

}