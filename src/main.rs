mod connection;
use connection::Server;
use std::io::Error;
use std::io::Write;
use std::io::Read;
use std::net::TcpStream;
use std::vec::Vec;
use std::string::String;
use std::str;


fn main() -> std::io::Result<()> {
    // let mut stream = TcpStream::connect("127.0.0.1:6379")?;
    let server : Server = Server::new(String::from("127.0.0.1"), 6379);
    let result : Result<TcpStream, Error> = server.connect();
    if let Ok(mut stream) = result {
        let mut buffer = [0; 100];    
        let mut v:Vec<String> = Vec::new();

        v.push(String::from("get"));
        v.push(String::from("a"));
        //v.push(String::from("hello world"));
        let str = build_cmd(v);
        stream.write(&str.into_bytes())?;
        stream.read(&mut buffer)?;

        println!("{:?}", str::from_utf8(&buffer).unwrap());
        Ok(())    
    } 
    else {
        panic!()
    }
}


fn build_cmd(v: Vec<String>) -> String {
    let start = format!("*{}\r\n", v.len());
    let mut res = String::new();
    res.push_str(&*start);
    for x in &v {
        let temp = format!("${}\r\n{}\r\n", x.len(), x);
        res.push_str(&*temp);
    }
    res
}

