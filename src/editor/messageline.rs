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

    pub fn get_msg_as_bytes(&self) -> Option<&String>
    {
        self.message.as_ref()
        
    }
}
