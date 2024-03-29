use crate::utils::{databuffer::DataBuffer, terminalcommands};

pub struct Statusline{
        pub height: usize,
        pub width: usize,
        x_pos:usize, //column
        y_pos: usize, // row or line number
        status_buf: DataBuffer
}

impl Statusline
{
    pub fn new(height:usize, width:usize, x_pos:usize, y_pos:usize) -> Self
    {
        Statusline{height, width, x_pos, y_pos, status_buf: DataBuffer::new()}
    }

    pub fn get_formatted_statusline(&mut self, file_name: &Option<String>, line_number:usize, col:usize) -> &DataBuffer
    {
        self.status_buf.clear();
        let inverted_redition = terminalcommands::graphic_rendition_cmd(terminalcommands::Rendition::INVERTEDCOLORS);
        let move_cmd = terminalcommands::move_cursor_command(self.x_pos, self.y_pos+2);
        let binding = "  [No Name]".to_string();
        let filename = file_name
            .as_ref()
            .unwrap_or(&binding);
        self.status_buf.append_all(inverted_redition.as_bytes());
        self.status_buf.append_all(" ".as_bytes());
        self.status_buf.append_all(move_cmd.as_bytes());
        self.status_buf.append_all(filename.as_bytes());
        let cursor_string = format!("[{}:{}]  ", line_number, col);

        let start_idx = 2 + filename.len();
        let end_idx = self.width - (2 + cursor_string.len());
        for  _ in start_idx..end_idx
        {
           self.status_buf.append(b' '); 
        }
        self.status_buf.append_all(cursor_string.as_bytes());
        self.status_buf.append_all("  ".as_bytes());
        self.status_buf.append_all(terminalcommands::graphic_rendition_cmd(terminalcommands::Rendition::DEFAULT).as_bytes());

        &self.status_buf

    }
    
}

