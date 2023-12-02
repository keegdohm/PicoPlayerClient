#![warn(clippy::pedantic)]
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;


mod buffer;

use buffer::AudioBuffer;

use id3::{Tag, TagLike};



fn get_metadata(){
   let tag = Tag::read_from_path("/Users/keegan/Msd/Capstone/pico_player_client/Mr_Blue_Sky-Electric_Light_Orchestra.mp3").unwrap();
   // Get a bunch of s frames...
   if let Some(artist) = tag.artist() {
       println!("artist: {}", artist);
   }
   if let Some(title) = tag.title() {
       println!("title: {}", title);
   }
   if let Some(album) = tag.album() {
       println!("album: {}", album);
   }
  // if let Some(duration) = tag.duration(){
  //     println!("duration: {}",Duration::as_secs(duration));
  // }

   // Get frames before getting their content for more complex tags.
  // if let Some(artist) = tag.get("TPE1").and_then(|frame| frame.content().text()) {
  //     println!("artist: {}", artist);
  // }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Connect to a peer
    let result = TcpStream::connect("192.168.5.166:1234").await;
    let mut buf = AudioBuffer::new("/Users/keegan/Msd/Capstone/Mr_Blue_Sky-Electric_Light_Orchestra-trimmed.mp3".to_string()).unwrap();
        match result {
            Ok(mut stream) => {
                // let mut buf = [0u8;4097];
                // buf[4096] = 1;
                // let buf:[u8;6] = [0,1,2,3,4,5];
                match  buf.transmit(&mut stream).await {
                // match stream.write_all(&buf).await{
                    Ok(_) => {
                        println!("Buffer sent successfully!");
                    },
                    Err(e) => println!("Error writing to the stream: {}", e),
                }
            },
            Err(e) => println!("Failed to connect to the server: {}", e),
        }
    Ok(())
}
