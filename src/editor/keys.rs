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
    Quit,
    ArrowKey(ArrowType),
    Ctrlkey(CtrlKeys),
    Other(u8)
}
