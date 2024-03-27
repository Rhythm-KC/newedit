use std::io::{self, Write};
use std::{fs::File, io::BufReader};

use crate::editor::{footer::Footer, keys::Key, textwindow::TextWindow};
use crate::editor::inputhandler;
use crate::utils::terminalcommands;
use crate::utils::databuffer::DataBuffer;



pub struct Editor<'a>
{

    text_window: &'a mut TextWindow<'a>,
    footer: &'a mut Footer<'a>,
    editor_buffer: DataBuffer,
    output: &'a mut io::Stdout
}

impl<'a> Editor<'a>
{

    pub fn new(text_window: &'a mut TextWindow<'a>, footer: &'a mut Footer<'a>, output: &'a mut io::Stdout) -> Self
    {
        Editor{text_window, footer, editor_buffer:DataBuffer::new(), output}
    }

    pub fn open_editor(&mut self, file_name: Option<String>)
    {
        let bufreader : Option<&mut BufReader<File>> = None;
        if file_name.is_none(){
            let _ = self.text_window.open_text_window(bufreader).map_err(|msg| self.post_msg(msg.to_string()));
        }

        let file = File::open(file_name.as_ref().unwrap());
        if file.is_err()
        {
            self.post_msg(format!("Cannot open file:{}", file_name.unwrap()));
            return;
        }
        let bufreader = &mut BufReader::new(file.unwrap());
        match self.text_window.open_text_window(Some(bufreader)){
            Ok(msg)  => self.post_msg(msg.to_string()),
            Err(msg)  => self.post_msg(msg.to_string()),
        };
        
    }
    
    fn post_msg(&mut self, msg: String)
    {
        self.footer.post_message(msg);
    }


    fn map_keys(&mut self, key : Key) -> Result<(), &'static str>{
        self.text_window.process_key_press_for_text_window(key)
    }


    pub fn process_input(&mut self){
        let key = inputhandler::process_key_press();
        match key{
            Ok(key) => {let _ = self.map_keys(key).map_err(|err_msg| self.post_msg(err_msg.to_string()));},
            Err(e) => {self.post_msg(e.to_string())}
        }
    }

    pub fn render_editor(&mut self) -> io::Result<()>
    {
        self.editor_buffer.append_all(terminalcommands::hide_cursor().as_bytes());
        self.editor_buffer.append_all(terminalcommands::move_cursor_to_top().as_bytes());
        self.text_window.add_window_context_to_buffer(&mut self.editor_buffer);
        self.footer.add_footer_to_buffer(&mut self.editor_buffer);
        self.editor_buffer.append_all(terminalcommands::move_cursor_command(1, 1).as_bytes());
        let mut handle = self.output.lock();
        handle.write(self.editor_buffer.empty_the_buffer().as_slice())?;
        handle.flush()?;
        self.editor_buffer.append_all(terminalcommands::show_cursor().as_bytes());
        self.editor_buffer.clear();
        Ok(())
    }
    
}


