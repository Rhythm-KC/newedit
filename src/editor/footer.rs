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

    pub(crate) fn add_footer_to_buffer(&mut self, editor_buffer: &mut DataBuffer, cx: usize, cy: usize)
    {
        editor_buffer.append_all(self.statusline.get_formatted_statusline(&self.filename.cloned(), cx, cy).get_data().as_slice());
        self.messageline.get_formatted_message(editor_buffer);
    }

}
