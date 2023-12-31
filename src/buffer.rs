#![warn(clippy::pedantic)]
extern crate alloc;

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
    empty: bool,
}

impl AudioBuffer {
    pub fn new(file_path: String) -> Result<Self, io::Error> {
        let mut file = File::open(&file_path)?;
        let metadata = file.metadata()?;
        let mut default_data = [0u8; 4096];
        let _ = file.read_exact(&mut default_data);
        let default_index = 0;
        let default_empty = metadata.len() == 0;

        Ok(Self {
            data: default_data,
            path: file_path,
            index: default_index,
            empty: default_empty,
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
        let file = self.open().unwrap(); 
        if file.metadata().unwrap().len() - self.index >= 4096 {
            self.read_packet(file);

        } else {
            self.read_remaining();
            self.empty = true;
        }
    }

    pub fn read_packet(&mut self, file: File){
        match file.read_exact_at(&mut self.data, self.index) {
                Ok(_) => {
                    self.index += self.data.len() as u64;
                    println!("Read to Byte: {}", self.index);
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

    }
    fn read_remaining(&mut self){
        self.data = [0;4096];
        let file = self.open().unwrap();
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
        let mut counter = 0;
        while !self.is_empty() {
            println!("writing file to stream");
            match stream.write_all(self.get_data()).await {
                Ok(_) => {
                    println!("Count = {}", counter);
                    self.next();
                    counter += 1;
                },
                Err(e) => {
                    println!("Error writing to the stream: {}", e);
                    return Err(e); // Return the error
                }
            }
        }
        Ok(()) // Return Ok(()) at the end of successful transmission
    } 


    // async fn get_response(stream: &mut TcpStream) {
    //     loop {
    //         let mut buffer = [0u8; 4096];
    //         match stream.read(&mut buffer).await {
    //             Ok(bytes_read) => {
    //                 if bytes_read == 0 {
    //                     println!("Connection closed by peer.");
    //                     break;
    //                 }
    //                 let response = from_utf8(&buffer[..bytes_read]).unwrap();
    //                 if response == "continue" {
    //                     break;
    //                 }
    //                 else {
    //                     println!("{}", response);
    //                 }
    //             },
    //             Err(e) => {
    //                 println!("Some error occurred: {}", e);
    //                 break;
    //             }
    //         }
    //     }
    // }

}

