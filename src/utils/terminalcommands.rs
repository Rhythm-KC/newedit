static ESC_SEQ: &str = "\x1b"; 

pub enum Rendition
{
    BOLD,
    UNDERLINE,
    DEFAULT,
    INVERTEDCOLORS,
    BLINK,
}

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
    format!("{}[{};{}H",ESC_SEQ, x_coord, y_coord)
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

pub fn bold_cursor() -> String
{
    create_command("[2 q")
}

pub fn bar_cursor()-> String
{
    create_command("[6 q")
}

pub fn graphic_rendition_cmd(rendition_type: Rendition) -> String
{
    let formatter = |code:u8| format!("{}[{}m", ESC_SEQ, code);
    match rendition_type{
        Rendition::DEFAULT => {formatter(0)},
        Rendition::BOLD=> {formatter(1)},
        Rendition::UNDERLINE=> {formatter(4)},
        Rendition::BLINK=> {formatter(5)},
        Rendition::INVERTEDCOLORS=> {formatter(7)},
    }
}
