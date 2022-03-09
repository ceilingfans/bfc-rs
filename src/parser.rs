use std::string::ParseError;

#[derive(Debug, PartialEq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    CellShift {
        amount: i8,
        loc: Option<Location>,
    },
    PointerShift {
        amount: i8,
        loc: Option<Location>,
    },
    Read {
        loc: Option<Location>,
    },
    Write {
        loc: Option<Location>,
    },
    Loop {
        body: Vec<Node>,
        loc: Option<Location>,
    },
}