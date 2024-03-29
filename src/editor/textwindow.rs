use std::{cmp::min, fs::File, io::{BufReader, BufRead}};

use crate::{editor::{editorstate::EditorState, keys::Key, leftline::Leftline}, utils::{databuffer::DataBuffer, terminalcommands::{carriage_return, earase_in_line}}};


pub struct TextWindow<'a>{
    height: usize,
    width:usize,
    sidebar: &'a Leftline,
    editorstate: &'a mut EditorState<'a>
}

impl<'a> TextWindow<'a>
{
    pub fn new(height: usize, width: usize, sidebar: &'a Leftline, editorstate: &'a mut EditorState<'a>) -> Self
    {
        TextWindow{height, width, sidebar, editorstate}

    }

    pub fn add_window_context_to_buffer(&mut self, buffer: &mut DataBuffer)
    {
        for i in 0.. min(self.editorstate.num_rows, self.height){
            self.sidebar.add_left_line_data_to_buf(buffer);
            self.editorstate.fill_data_buffer_with_line(i, buffer);
            buffer.append_all(earase_in_line().as_bytes());
            buffer.append_all(carriage_return().as_bytes());
        }
    }

    pub fn process_key_press_for_text_window(&mut self, key: Key) -> Result<(), &'static str>
    {

        self.editorstate.map_keys(key)
    }

    fn open_empty_text_window(&mut self)
    {
        let _ = self.editorstate.add_row("".as_bytes());

    }
    pub fn open_text_window(&mut self, file_reader: Option<&mut BufReader<File>>) -> Result<&'static str, &'static str> 
    {
        if let Some(reader) = file_reader{
            for line in reader.lines()
            {
                let line = line.map_err(|_| "Error reading lines")?;
                self.editorstate.add_row(line.as_bytes()).map_err(|_|"Error adding lines to buffer")?;
            }
            Ok("File Opened ")
        }
        else
        {
            self.open_empty_text_window();
            Ok("Opening a new File")           
        }
    }
    pub fn get_line_numbers(&self) -> (usize, usize)
    {
        self.editorstate.get_line_numbers()
    }
    pub fn get_cursor_pos(&self) -> (usize, usize)
    {
        self.editorstate.get_cursor_position()
    }
    pub fn is_running(&self) -> bool
    {
        self.editorstate.is_running()
    }
    
}
