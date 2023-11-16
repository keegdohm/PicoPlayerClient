use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use rmp3::{RawDecoder,Frame,Sample,MAX_SAMPLES_PER_FRAME};
use std::fs::File;
use std::io::{self,Read};

fn send_file_byte_by_byte(file_path: &str, audio_buf: &mut AudioBuffer) -> io::Result<()> {
    // Open the file
    let mut file = File::open(file_path)?;
    // Create a buffer to read bytes into
    let mut buf = [0u8; 1];
    
    loop {
        match file.read_exact(&mut buf){
            Ok(_) => {
                println!("Byte read: {}", buf[0]);
                if audio_buf.size < 4096 {
                    audio_buf.buf[i] = buf[0];
                    audio_buf.size += 1;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
    Ok(())
}

fn decode_frame(start: usize, audio_buf: &[u8; 4096], output_buf: &[Sample::default(); MAX_SAMPLES_PER_FRAME]) -> (Frame, usize) {
    // need to check to see if the audio buffer has data in it
    let mut decoder = RawDecoder::new();
    let (frame, bytes_consumed) = decoder.next(audio_buf, &mut output_buf).unwrap();
    return (frame, bytes_consumed);
}

fn decode_buffer(audio_buf: &[u8; 4096], output_buf: &[Sample::default(); MAX_SAMPLES_PER_FRAME]){
    
}
fn play_audio(audio_buf: &[u8; 4096]){

}
fn main() {
    let mut buf =  [0; 4096];
    if let Err(err) = send_file_byte_by_byte("/Users/keegan/Msd/Capstone/pico_player_source/sound.mp3", &mut buf) {
        eprintln!("Error: {:?}", err);
    }
    let (_frame, bytes_consumed) = decode_frame(&buf);
    println!("{}", bytes_consumed);
}
