use crate::utils::databuffer::DataBuffer;

pub struct Statusline{
        pub height: usize,
        pub width: usize,
        pub(super)x_pos:usize,
        pub(super)y_pos: usize,
        pub(super)status_buf: DataBuffer
}

impl Statusline
{
    pub fn new(height:usize, width:usize, x_pos:usize, y_pos:usize) -> Self
    {
        Statusline{height, width, x_pos, y_pos, status_buf: DataBuffer::new()}
    }
}

