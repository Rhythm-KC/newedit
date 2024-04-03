use std::{cmp::{max, min}, io::Read};


use crate::{editor::keys::{ArrowType, CtrlKeys, Key}, utils::databuffer::{self, DataBuffer}};

pub struct EditorWindow{
    width: usize,
    height: usize,
    x_min_pos: usize, // col
    y_min_pos: usize, // row line number
}

pub struct EditorState<'a>{
    // current cursor position with respect to the entire terminal
    cx: usize, // the column position in  on the editor window
    cy: usize, // row position on the text window min can be 0 max can go up to the number of col
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
        EditorState{cx: 0, cy: 0, num_rows:0, rows: Vec::new(), window, open:true}
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
            Key::DisplayKey(val) => {self.insert_text(val)},
            Key::ArrowKey(arrowtype) => {self.move_cursor(arrowtype)}
            Key::Ctrlkey(ctrlkey)=> {self.move_ctrl_keys(ctrlkey)}
            Key::Quit =>{self.open = false; Ok(())}
            Key::Backspace => {self.delete_text()}
            _ =>{Ok(())}
        }

    }

    fn move_up(&mut self) -> Result<(), &'static str>{
        if self. cy > self.window.y_min_pos {
            self.cy -= 1;
            self.validate_row_cursor_position();
        }
        else
        {
            self.cy = self.window.y_min_pos;
        }
        Ok(())
    }

    fn move_down(&mut self) -> Result<(), &'static str> {
        if self.cy == usize::max_value(){
            return Err("Max limit reached");
        }
        if self.cy < self.num_rows - 1{
            self.cy += 1;
        }
        self.validate_row_cursor_position();

        Ok(())
    }

    fn move_left(&mut self) -> Result<(), &'static str>{
        if self.cx > 0{
            self.cx -= 1;
        }
        else{
            self.move_up()?;
            if self.cy !=0
            {
                self.move_to_end_of_row()?;

            }
        }
        Ok(())
    }

    fn move_right(&mut self) -> Result<(), &'static str>{
        if let Some(row) = self.rows.get(self.cy)
        {
            let max_right = row.get_data().len();
            if self.cx < max_right{
                self.cx += 1;
            }
            else
            {
                self.move_down()?;
                self.move_to_start_of_the_row()?;
            }

            }
            Ok(())
    }
    fn move_to_end_of_row(&mut self)-> Result<(), &'static str>{
        if let Some(row) = self.rows.get(self.cy)
        {
            self.cx = row.get_data().len();
            self.validate_row_cursor_position();

        }
        Ok(())
    }

    fn move_to_start_of_the_row(&mut self)-> Result<(), &'static str>{
        self.cx = 0;
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
        if self.cx > self.window.width - 1{
            return self.cx - self.window.width + 1;
        }
        0
    }

    fn validate_row_cursor_position(&mut self){
        if let Some(row) = self.rows.get(self.cy)
        {
            if self.cx > row.get_data().len(){
                self.cx = row.get_data().len();
            }
        }
    }

    pub fn fill_data_buffer_with_line(&self, row:usize, buf: &mut DataBuffer)-> Result<(), String>
    {
        let row_index = min(self.rows.len(), row+self.get_row_offset());
        let row = match self.rows.get(row_index)
        {
            Some(data) => data,
            None => {
                return Err(format!("Error reading row: {}", row_index));
            },
        };
        
        let col_offset = self.get_col_offset();
        if col_offset >= row.len(){
            return Ok(())
        }
        let char_index = min(row.len(), self.window.width + col_offset);
        buf.append_all(&row.get_data()[col_offset..char_index]);
        return Ok(())
    }

    pub fn get_line_numbers(&self) -> (usize, usize)
    {
        (self.cy + 1, self.cx + 1,)
    }

    pub fn get_cursor_position(&self) -> (usize, usize)
    {
        (min(self.cy+1, self.window.height), self.cx + self.window.x_min_pos)
    }
    
    fn delete_text(&mut self) -> Result<(), &'static str>
    {
        if self.cx == 0
        {
            return Ok(());
        }
        if let Some(line) = self.rows.get(self.cy)
        {
            let mut prev_line = line.get_data()[0..self.cx-1].to_vec();
            let after_cursor = &line.get_data()[self.cx..line.len()];
            prev_line.extend_from_slice(after_cursor);
            let mut buf = DataBuffer::new();
            buf.append_all(&prev_line);
            self.rows[self.cy] = buf;
            self.move_left()?
       }
        
        Ok(())
    }


    fn insert_text(&mut self, char: u8) ->Result<(), &'static str>{
        let  data = self.rows.get(self.cy);
        match data
        { 
            Some(line)=>
            {
                let prev_state = &line.get_data().clone()[0..self.cx];
                let mut lstate = prev_state.to_vec(); 
                lstate.push(char);
                let end = &line.get_data().clone()[self.cx..line.get_data().len()];
                lstate.extend_from_slice(end);
                let mut buf = DataBuffer::new();
                buf.append_all(&lstate);
                self.rows[self.cy] = buf;
                self.move_right()

            },
            None=>{Ok(())}
        }
    }

    
    pub fn is_running(&self) -> bool
    {
        self.open
    }


}
