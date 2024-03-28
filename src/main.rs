mod terminal;
mod editor;
mod utils;

use std::env;

use std::io;
use std::usize;


use crate::terminal::terminal::Terminal;
use crate::editor::{editor::Editor, editorstate::{EditorState, EditorWindow}, footer::Footer, leftline::Leftline, messageline::Messageline, statusline::Statusline, textwindow::TextWindow};

fn main()
{
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    let terminal = Terminal::enable_raw();
    if terminal.is_err(){
        eprint!("Error reading enabling terminal");
        return;
    }
    let terminal = terminal.unwrap();
        
    let (terminal_height, terminal_width) = terminal.size();

    let footer_height = 2;
    let footer_width = terminal_width;

    let messageline_height = (footer_height/2) as usize;
    let statusline_hight = footer_height - messageline_height;
    
    let messageline_row_cordi = terminal_height - footer_height + 1;
    let statusline_row_cordi = terminal_height;

    let statusline = Statusline::new(statusline_hight, footer_width, statusline_row_cordi, 0);
    let mut messageline = Messageline::new(messageline_height, footer_width, messageline_row_cordi, 0);
    let mut footer = Footer::new(&statusline, &mut messageline);

    // TEXT Window with all the dependencies
    let text_window_height = terminal_height - footer_height;
    let text_window_widht = terminal_width;
    let leftline_width = 2;
    let editorstate_widht = text_window_widht - leftline_width;
    let editorstate_min_col_cordi = leftline_width + 1;

    let leftline = Leftline::new(text_window_height, leftline_width);
    let window = EditorWindow::new(editorstate_widht, text_window_height, 0, editorstate_min_col_cordi);
    let mut editorstate = EditorState::new(&window);
    let mut textwindow = TextWindow::new(text_window_height, text_window_widht, &leftline, &mut editorstate);
    let mut io = io::stdout();

    let mut editor = Editor::new(&mut textwindow, &mut footer, &mut io);
    editor.open_editor(filename);
    loop
    {
        editor.process_input();
        let _ = editor.render_editor();
    }

}
