static ESC_SEQ: &str = "\x1b"; 
fn create_command(cmd: &'static str) -> String
{
    format!("{}{}", ESC_SEQ, cmd)
}
pub fn clear_entire_screen() -> String
{
    create_command("[2J")
}

pub fn move_cursor_command(x_coord:usize, y_coord:usize) -> String
{
    format!("{}[{}:{}H",ESC_SEQ, x_coord, y_coord)
}

pub fn earase_in_line() -> String
{
    create_command("[K")
}
pub fn carriage_return() -> String
{
    String::from("\r\n")
}

pub fn hide_cursor() -> String
{
    create_command("[?25l")
}

pub fn move_cursor_to_top() -> String
{
    create_command("[H")
}

pub fn show_cursor() -> String
{
    create_command("[?25h")
}
