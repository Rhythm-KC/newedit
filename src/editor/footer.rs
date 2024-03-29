use crate::{editor::{messageline::Messageline, statusline::Statusline}, utils::databuffer::DataBuffer};


pub struct Footer<'a>{
    filename: Option<&'a String>,
    statusline: &'a mut Statusline,
    messageline: &'a mut Messageline
}

impl<'a> Footer<'a>{
    
    pub fn new(statusline: &'a mut Statusline, messageline: &'a mut Messageline, filename: Option<&'a String>) -> Self{
        Footer{filename, statusline, messageline}
    }


    pub fn post_message(&mut self, msg: String){
        self.messageline.add_message(msg)
    }

    pub(crate) fn add_footer_to_buffer(&self, editor_buffer: &mut DataBuffer)
    {
    }

}
