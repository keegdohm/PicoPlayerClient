
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use rmp3::{RawDecoder, Frame, Sample, MAX_SAMPLES_PER_FRAME};
use std::str::{from_utf8};
mod buffer;
use buffer::AudioBuffer;
use tokio::io::AsyncReadExt;

fn decode_frame<'a>(start: usize, audio_buf: &'a [u8; 4096], mut output_buf: &'a [Sample; MAX_SAMPLES_PER_FRAME]) -> (Frame<'a, 'a>, usize) {
    let mut decoder = RawDecoder::new();
    let (frame, bytes_consumed) = decoder.next(audio_buf, &output_buf).unwrap();
    return (frame, bytes_consumed);
}

async fn write_buffer(stream: &mut TcpStream, buf: &mut AudioBuffer) {
    while !buf.is_empty() {
        match stream.write_all(buf.get_data()).await {
            Ok(_) => {
                get_response(stream).await;
                buf.next();
            },
            Err(e) => println!("Error writing to the stream: {}", e),
        }
    }
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
            },
            Err(e) => {
                println!("Some error occurred: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let result = TcpStream::connect("192.168.5.165:1234").await;
    let mut buf = AudioBuffer::new("your_audio_file_path".to_string()).unwrap();

    match result {
        Ok(mut stream) => {
            match stream.write_all(b"hello world!").await {
                Ok(_) => {
                    println!("Buffer sent successfully!");
                    get_response(&mut stream).await; // Wait for a 'continue' response from the server
                },
                Err(e) => println!("Error writing to the stream: {}", e),
            }
        },
        Err(e) => println!("Failed to connect to the server: {}", e),
    }

    Ok(())
}

