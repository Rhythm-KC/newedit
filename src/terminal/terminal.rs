use termios::{tcsetattr, Termios,VMIN, VTIME, CS8, BRKINT, INPCK, ISTRIP, ECHO, ICANON, ICRNL, IEXTEN, ISIG, IXON, OPOST, TCSAFLUSH};
use std::{io::{self, stdout, Write}, os::fd::AsRawFd};
use termion;

use crate::utils::terminalcommands;

pub struct Terminal{
    row: usize,
    col:usize,
    original_termios: Termios,
    _current_termios: Termios,

}
impl Terminal 
{
    pub fn enable_raw() -> io::Result<Self>
    {
            let fp = io::stdin().as_raw_fd();
            let original_termios = Termios::from_fd(fp)?;
            let mut raw = original_termios;
            raw.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
            raw.c_oflag &= !(OPOST);
            raw.c_cflag |= CS8;
            raw.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
            raw.c_cc[VMIN] = 0;
            raw.c_cc[VTIME] = 1;
            tcsetattr(fp, TCSAFLUSH, &raw)?;

        // get terminal size
        let screen_size = termion::terminal_size()?;
        let (col, row) = screen_size;

        
        Ok(Terminal{row: (row as usize), col: (col as usize), original_termios, _current_termios:raw})
    }

    pub fn size(&self) -> (usize, usize){
        (self.row, self.col)
    }
}
 
impl Drop for Terminal 
{
    fn drop(&mut self)
    {
        // clearing all the screen things
        let clear_screen = terminalcommands::clear_entire_screen();
        let move_to_top = terminalcommands::move_cursor_to_top();
        let mut io = stdout().lock();
        let _ = io.write(clear_screen.as_bytes());
        let _ = io.write(move_to_top.as_bytes());
        let _ = io.flush(); 
        

        // restoring the original terminal state
        let fp = io::stdin().as_raw_fd();
        tcsetattr(fp, TCSAFLUSH, &self.original_termios).expect("could not apply setting");

    }
}
