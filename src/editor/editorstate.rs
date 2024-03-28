use std::cmp::{max, min};


use crate::{editor::keys::{ArrowType, CtrlKeys, Key}, utils::databuffer::DataBuffer};

pub struct EditorWindow{
    width: usize,
    height: usize,
    x_min_pos: usize,
    y_min_pos: usize,
}

pub struct EditorState<'a>{
    // current cursor position with respect to the entire terminal
    cx: usize,
    cy: usize,
    rows: Vec<DataBuffer>,
    pub num_rows: usize,
    window: &'a EditorWindow,
    open: bool,
}

impl EditorWindow{
    pub fn new(width: usize, height: usize, x_min_pos: usize, y_min_pos: usize) -> Self{
        EditorWindow{width, height, x_min_pos, y_min_pos}
    }
}

impl<'a> EditorState<'a>{

    pub fn new(window: &'a EditorWindow) -> Self{
        EditorState{cx: window.x_min_pos, cy: window.y_min_pos, num_rows:0, rows: Vec::new(), window}
    }

    fn move_cursor(&mut self, arrow: ArrowType) -> Result<(), &'static str>{
        match arrow {
            ArrowType::UP =>{self.move_up()},
            ArrowType::DOWN =>{self.move_down()},
            ArrowType::LEFT =>{self.move_left()},
            ArrowType::RIGHT =>{self.move_right()},
        }
    }

    fn move_ctrl_keys(&mut self, ctrl: CtrlKeys) -> Result<(), &'static str>{
        
        match ctrl{
            CtrlKeys::End =>{self.move_to_end_of_row()},
            CtrlKeys::Home => {self.move_to_start_of_the_row()},
            CtrlKeys::PageUp => {self.go_to_start_page()},
            CtrlKeys::PageDown => {self.go_to_end_page()},
        }
    }

    pub fn map_keys(&mut self, key: Key)-> Result<(), &'static str>{
        match key {
            Key::Other(_val) => {Ok(())},
            Key::ArrowKey(arrowtype) => {self.move_cursor(arrowtype)}
            Key::Ctrlkey(ctrlkey)=> {self.move_ctrl_keys(ctrlkey)}
            Key::Quit =>{self.open = false; Ok(())}
            _ =>{Ok(())}
        }

    }

    fn move_up(&mut self) -> Result<(), &'static str>{
        if self.cy > self.window.y_min_pos{
            self.cy -= 1;
            self.validate_row_cursor_position();
            return Ok(());
        }
        Err("Invalid Editor state")
    }

    fn move_down(&mut self) -> Result<(), &'static str> {
        if self.cy == usize::max_value(){
            return Err("Max limit reached");
        }
        if self.cy != max(self.num_rows, self.window.height) - 1{
            self.cy += 1;
        }
        self.validate_row_cursor_position();

        Ok(())
    }

    fn move_left(&mut self) -> Result<(), &'static str>{
        if self.cx > self.window.x_min_pos{
            self.cx -= 1;
        }
        else{
            self.move_up()?;
            self.move_to_end_of_row()?;
        }
        Ok(())
    }

    fn move_right(&mut self) -> Result<(), &'static str>{
        let max_right = self.rows[self.cy].get_data().len() + 1;
        if self.cx < max_right{
            self.cx += 1;
        }
        else
        {
            self.move_down()?;
            self.move_to_start_of_the_row()?;
        }
        Ok(())
    }
    fn move_to_end_of_row(&mut self)-> Result<(), &'static str>{
        self.cx = self.rows[self.cy].get_data().len() + 1;
        self.validate_row_cursor_position();
        Ok(())
    }

    fn move_to_start_of_the_row(&mut self)-> Result<(), &'static str>{
        self.cx = self.window.x_min_pos;
        self.validate_row_cursor_position();
        Ok(())
    }
    fn go_to_start_page(&mut self)-> Result<(), &'static str>{
        self.cy = self.window.y_min_pos;
        self.validate_row_cursor_position();
        Ok(())
    }

    fn go_to_end_page(&mut self)-> Result<(), &'static str>{
        self.cy = self.num_rows - 1;
        self.validate_row_cursor_position();
        Ok(())
    }

    pub fn add_row(&mut self, data: &[u8]) -> Result<(), &'static str>{
        self.rows.push(DataBuffer::create(data));
        self.num_rows += 1;
        Ok(())
    }

    fn get_row_offset(&self) -> usize{
        if self.cy >= self.window.height{
            return self.cy - self.window.height + 1;
        }
        0
    }

    fn get_col_offset(&self) -> usize{
        if self.cx >= self.window.width{
            return self.cx - self.window.width + 1;
        }
        0
    }

    fn validate_row_cursor_position(&mut self){
        if self.cx > self.rows[self.cy].get_data().len() + 1{
            self.cx = self.rows[self.cy].get_data().len() + 1
        }
    }

    // returns the x coordinate and the y coordinate on the screen 
    pub fn get_cur_pos(&self ) -> (usize, usize){
        (self.cx + self.window.x_min_pos, self.cy)
    }

    pub fn fill_data_buffer_with_line(&self, row:usize, buf: &mut DataBuffer)
    {
        let row_index = min(self.rows.len(), row+self.get_row_offset());
        let row = self.rows.get(row_index).unwrap();
        let col_offset = self.get_col_offset();
        if col_offset >= row.len(){
            return
        }
        let char_index = col_offset + min(row.len() - col_offset, self.window.width);
        buf.append_all(&row.get_data()[col_offset..char_index]);
    }

    pub fn is_running(&self) -> bool
    {
        self.open
    }


}
