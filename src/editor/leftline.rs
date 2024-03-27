use crate::utils::databuffer::DataBuffer;

pub struct Leftline{
    pub height: usize,
    pub width: usize,
    line_buf: DataBuffer
}

impl Leftline{
    pub fn new(height: usize, width:usize) ->Self{
        Leftline{height, width, line_buf: DataBuffer::new()}
    }

    pub fn add_left_line_data_to_buf(&self, buf: &mut DataBuffer)
    {
        buf.append(b'~');
        if self.width > 1
        {
            let padding = " ".repeat(self.width - 1);
            buf.append_all(padding.as_bytes());
        }

    }
}
