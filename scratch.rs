//FIRMWARE
//
//

//decode a 4KB AudioBuffer
fn decode_buf(&self) -> rmp3::Frame {}
fn decode_frame<'src, 'dst>(
    start: usize,
    audio_buf: &'src [u8; 4096],
    output_buf: &'dst mut [Sample; MAX_SAMPLES_PER_FRAME],
) -> (Frame<'src, 'dst>, usize) {
    RawDecoder::new().next(audio_buf, output_buf).unwrap()
} 
