#![warn(clippy::pedantic)]
extern crate alloc;
use alloc::vec::Vec;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::prelude::FileExt;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use core::str::from_utf8;
pub struct AudioBuffer {
    data: [u8; 4096],
    index: u64,
    path: String,
    length: u64,
    empty: bool,
    bytes_sent: u64,
}

impl AudioBuffer {
    pub fn new(file_path: String) -> Result<Self, io::Error> {
        let mut file = File::open(&file_path)?;
        let metadata = file.metadata()?;
        let default_data = [0u8; 4096];
        let default_index = 0;
        let default_length = metadata.len();
        let default_empty = metadata.len() == 0;
        let default_bytes_sent = 0;

        Ok(Self {
            data: default_data,
            path: file_path,
            index: default_index,
            length: default_length,
            empty: default_empty,
            bytes_sent: default_bytes_sent,
        })
    }
    pub fn open(&mut self) -> Option<File> {
        match File::open(&self.path){
            Ok(file) => Some(file),
            Err(_) => None,
        }
    }

    pub fn next(&mut self) {
        // Open the file
        let mut file = self.open().unwrap();
        
        if file.metadata().unwrap().len() - self.index >= 4096 {
            match file.read_exact_at(&mut self.data, self.index) {
                Ok(_) => {
                    println!("Byte read: {}", self.data[0]);
                    self.index += self.data.len() as u64;
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        println!("Reached EOF!");
                    }
                    _ => {
                        println!("Error occurred updating the buffer after byte # {}", self.index);
                    }
                },
            }
        } else {
            self.read_packet();
            self.empty = true;
        }
    }

    fn read_packet(&mut self){
        self.data = [0;4096];
        let mut file = self.open().unwrap();
        match file.read_at(&mut self.data, self.index){
            Ok(_) => {
            },
            Err(err) => match err.kind(){
                io::ErrorKind::UnexpectedEof => {
                    println!("Reached EOF!");
                },
                _ => {
                    println!("Error occurred updating the buffer after byte # {}", self.index);
                },
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.empty
    }
    pub fn get_data(&self) ->  &[u8; 4096]{
        &self.data
    }

    pub async fn transmit(&mut self, stream: &mut TcpStream) -> Result<(), io::Error> {
        while !self.is_empty() {
            match stream.write_all(self.get_data()).await {
                Ok(_) => {
                    Self::get_response(stream).await; // Propagate errors from get_response
                    self.next();
                },
                Err(e) => {
                    println!("Error writing to the stream: {}", e);
                    return Err(e); // Return the error
                }
            }
        }
        Ok(()) // Return Ok(()) at the end of successful transmission
    } 


    async fn get_response(stream: &mut TcpStream) {
        loop {
            let mut buffer = [0u8; 4096];
            match stream.read(&mut buffer).await {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        println!("Connection closed by peer.");
                        break;
                    }
                    let response = from_utf8(&buffer[..bytes_read]).unwrap();
                    if response == "continue" {
                        break;
                    }
                    else {
                        println!("{}", response);
                    }
                },
                Err(e) => {
                    println!("Some error occurred: {}", e);
                    break;
                }
            }
        }
    }

}

