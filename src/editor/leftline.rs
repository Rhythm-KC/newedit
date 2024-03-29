
pub struct Leftline{
    pub height: usize,
    pub width: usize,
}

impl Leftline{
    pub fn new(height: usize, width:usize) ->Self{
        Leftline{height, width}
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
