pub enum ArrowType{
    
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub enum CtrlKeys{
    Home,
    End,
    PageUp,
    PageDown,
}

pub enum Key{
    Delete,
    EscapeKey,
    Backspace,
    Quit,
    ArrowKey(ArrowType),
    Ctrlkey(CtrlKeys),
    DisplayKey(u8),
    Other()
}
