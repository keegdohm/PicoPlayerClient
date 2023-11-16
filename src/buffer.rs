use rmp3::Frame;

struct Buffer<T, const SIZE: usize>{
    data: [T; SIZE],
    size: usize,
    is_active: boolean
}

impl Buffer{
    pub fn new_input() -> Self {
        let default_data = [0u8; 4096];
        let default_size = 0;
        Self <u8, 4096>{
            data: default_data,
            size: default_size,
        }
    }
    pub fn new_output() -> Self {
        let default_data = [Frame; 10];
        let default_size = 0;
        Self <Frame, 10>{
            data: default_data,
            size: default_size,
        }
    }
}



