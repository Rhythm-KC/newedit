use crate::utils::{databuffer::DataBuffer, terminalcommands};

pub struct Messageline{
    pub height:usize,
    pub width: usize,
    x_pos: usize,
    y_pos: usize,
    message: Option<String>
}

impl Messageline{
    pub fn new(height: usize, width: usize, x_pos: usize, y_pos: usize) -> Self {
        Messageline{ height, width, x_pos, y_pos, message: None}
    }

    pub fn add_message(&mut self, msg: String){
           self.message = Some(msg); 
    }
    pub fn get_formatted_message(&self, buf: &mut DataBuffer)
    {
        if let Some(message) = &self.message{
            let move_to_msg = terminalcommands::move_cursor_command(self.x_pos, self.y_pos + 2);
            buf.append_all(move_to_msg.as_bytes());
            buf.append_all(message.as_bytes());
        }

    }

}
