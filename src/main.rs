use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use rmp3::{RawDecoder,Sample,MAX_SAMPLES_PER_FRAME};
use std::fs::File;
use std::io::{self,Read};
fn send_file_byte_by_byte(file_path: &str, buffer: &mut [u8; 4096]) -> io::Result<()> {
    // Open the file
    let mut file = File::open(file_path)?;
    // Create a buffer to read bytes into
    let mut buf = [0u8; 1];
    let mut i: usize = 0;
    loop {
        match file.read_exact(&mut buf){
            Ok(_) => {
                println!("Byte read: {}", buf[0]);
                if i < 4096{
                    buffer[i] = buf[0];
                    i += 1;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
    Ok(())
}
fn receive_bytes(audio_buffer: &[u8; 4096]){
    let mut decoder = RawDecoder::new();
    let mut output_buf = [Sample::default(); MAX_SAMPLES_PER_FRAME];

        // pseudocode
    for byte in audio_buffer{
        let mut input_buf = [0u8;1];
        input_buf[0] = *byte;
        if let Some((frame, bytes_consumed)) = decoder.next(&mut input_buf, &mut output_buf) {
            println!("successfully decoded a byte!")
        }


        // do something with the frame

        // imaginary_file.skip(bytes_consumed);
    }

}
fn main() {
    let mut buf =  [0; 4096];
    if let Err(err) = send_file_byte_by_byte("/Users/keegan/Msd/Capstone/pico_player_source/sound.mp3", &mut buf) {
        eprintln!("Error: {:?}", err);
    }
    receive_bytes(&buf);
}
