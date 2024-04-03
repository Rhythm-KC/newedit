use std::io::{self, Read};
use crate::editor::keys::{ArrowType, CtrlKeys, Key};

const CTRL_MODIFIER: u8 = 0x1f;
const ESC_SQE : u8 = b'\x1b';

fn get_ctrl_key(c : u8) -> u8{
    c & CTRL_MODIFIER
}

fn read_from_stdin(buf: &mut [u8;1]) -> Result<usize, &'static str>{
    let read_bytes = io::stdin().read(buf);
    if read_bytes.is_err(){
        return Err("Failed to read input");
    };
    Ok(read_bytes.unwrap())
}


fn key_buffer_for_control_key(key: &u8) -> Result<Key, &'static str>{
    let mut key_buf = [0; 1];
    if read_from_stdin(&mut key_buf)? != 1{
            return Ok(Key::EscapeKey);
    }
    if key_buf[0] != b'~'{
        return Ok(Key::EscapeKey);
    }

    match key {
        b'1' => Ok(Key::Ctrlkey(CtrlKeys::Home)),
        b'3' => Ok(Key::Delete), 
        b'4' => Ok(Key::Ctrlkey(CtrlKeys::End)),

        b'5' => Ok(Key::Ctrlkey(CtrlKeys::PageUp)),
        b'6' => Ok(Key::Ctrlkey(CtrlKeys::PageDown)),
        b'7' => Ok(Key::Ctrlkey(CtrlKeys::Home)),
        b'8' => Ok(Key::Ctrlkey(CtrlKeys::End)),
        _ => Ok(Key::EscapeKey),
    }

}

pub fn process_key_press() -> Result<Key, &'static str>{
    let mut buf = [0; 1];
    let _ = read_from_stdin(&mut buf)?;
    
    if buf[0 ]== get_ctrl_key('q' as u8){
        return Ok(Key::Quit);
    }

    if buf[0] == ESC_SQE
    {
        let mut escape_buf = [0; 1];
        let mut key_buf = [0;1];
        if read_from_stdin(&mut escape_buf)? != 1{
            return Ok(Key::EscapeKey)
        }
        if read_from_stdin(&mut key_buf)? != 1{
            return Ok(Key::EscapeKey);
        }
        
        if escape_buf[0] == b'['
        {
            let key = key_buf[0];
            match key {
                key if ('0'..='9').contains(&(key as char)) => {
                    return key_buffer_for_control_key(&key_buf[0]);
                },
                b'A' => return Ok(Key::ArrowKey(ArrowType::UP)),
                b'B' => return Ok(Key::ArrowKey(ArrowType::DOWN)),
                b'C' => return Ok(Key::ArrowKey(ArrowType::RIGHT)),
                b'D' => return Ok(Key::ArrowKey(ArrowType::LEFT)),
                b'H' => return Ok(Key::Ctrlkey(CtrlKeys::Home)),
                b'F' => return Ok(Key::Ctrlkey(CtrlKeys::End)),
                _ => return Ok(Key::EscapeKey)
            }
        }
        else if escape_buf[0] == b'O' 
        {
           match key_buf[0]{
               b'H' => return Ok(Key::Ctrlkey(CtrlKeys::Home)),
               b'F' => return Ok(Key::Ctrlkey(CtrlKeys::End)),
                _ => return Ok(Key::EscapeKey)
           }; 
        }
        
        return Ok(Key::EscapeKey)

    }
    if buf[0] as u8 == 127
    {
        return Ok(Key::Backspace);
    }
    if buf[0].is_ascii_graphic() || buf[0].is_ascii_whitespace()
    {
       return Ok(Key::DisplayKey(buf[0]));
    }
    Ok(Key::Other())
}
