use self::Node::*;

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

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub loc: Location,
}

pub fn parse(source: &str) -> Result<Vec<Node>, ParserError> {
    let mut instructions = vec![];
    let mut stack = vec![];

    for (index, c) in source.chars().enumerate() {
        match c {
            '+' => instructions.push(CellShift {
                amount: 1,
                loc: Some(Location {
                    start: index,
                    end: index,
                }),
            }),
            '-' => instructions.push(CellShift {
                amount: -1,
                loc: Some(Location {
                    start: index,
                    end: index,
                }),
            }),
            '>' => instructions.push(PointerShift {
                amount: 1,
                loc: Some(Location {
                    start: index,
                    end: index,
                }),
            }),
            '<' => instructions.push(PointerShift {
                amount: -1,
                loc: Some(Location {
                    start: index,
                    end: index,
                }),
            }),
            '.' => instructions.push(Write {
                loc: Some(Location {
                    start: index,
                    end: index,
                }),
            }),
            ',' => instructions.push(Read {
                loc: Some(Location {
                    start: index,
                    end: index,
                }),
            }),
            '[' => {
                stack.push((instructions, index));
                instructions = vec![];
            }
            ']' => {
                if let Some((mut parent, open_index)) = stack.pop() {
                    parent.push(Loop {
                        body: instructions,
                        loc: Some(Location {
                            start: open_index,
                            end: index,
                        }),
                    });
                    instructions = parent;
                } else {
                    return Err(ParserError {
                        message: "Unmatched bracket pair".to_owned(),
                        loc: Location {
                            start: index,
                            end: index,
                        },
                    });
                }
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        let pos = stack.last().unwrap().1;
        return Err(ParserError {
            message: "Unmatched bracket pair".to_owned(),
            loc: Location {
                start: pos,
                end: pos,
            },
        });
    }

    Ok(instructions)
}

#[test]
fn parse_cell_shift() {
    assert_eq!(
        parse("+").unwrap(),
        [CellShift {
            amount: 1,
            loc: Some(Location { start: 0, end: 0 })
        }]
    );
    assert_eq!(
        parse("++").unwrap(),
        [
            CellShift {
                amount: 1,
                loc: Some(Location { start: 0, end: 0 }),
            },
            CellShift {
                amount: 1,
                loc: Some(Location { start: 1, end: 1 })
            }
        ]
    );
}

#[test]
fn parse_decrement() {
    assert_eq!(
        parse("-").unwrap(),
        [CellShift {
            amount: -1,
            loc: Some(Location { start: 0, end: 0 })
        }]
    );
    assert_eq!(
        parse("--").unwrap(),
        [
            CellShift {
                amount: -1,
                loc: Some(Location { start: 0, end: 0 }),
            },
            CellShift {
                amount: -1,
                loc: Some(Location { start: 1, end: 1 })
            }
        ]
    );
}

#[test]
fn parse_pointer_shift_increment() {
    assert_eq!(
        parse(">").unwrap(),
        [PointerShift {
            amount: 1,
            loc: Some(Location { start: 0, end: 0 })
        }]
    );
    assert_eq!(
        parse(">>").unwrap(),
        [
            PointerShift {
                amount: 1,
                loc: Some(Location { start: 0, end: 0 })
            },
            PointerShift {
                amount: 1,
                loc: Some(Location { start: 1, end: 1 })
            }
        ]
    );
}

#[test]
fn parse_pointer_shift_decrement() {
    assert_eq!(
        parse("<").unwrap(),
        [PointerShift {
            amount: -1,
            loc: Some(Location { start: 0, end: 0 })
        }]
    );
    assert_eq!(
        parse("<<").unwrap(),
        [
            PointerShift {
                amount: -1,
                loc: Some(Location { start: 0, end: 0 })
            },
            PointerShift {
                amount: -1,
                loc: Some(Location { start: 1, end: 1 })
            }
        ]
    );
}

#[test]
fn parse_read() {
    assert_eq!(
        parse(",").unwrap(),
        [Read {
            loc: Some(Location { start: 0, end: 0 })
        }]
    );
}

#[test]
fn parse_write() {
    assert_eq!(
        parse(".").unwrap(),
        [Write {
            loc: Some(Location { start: 0, end: 0 })
        }]
    );
}

#[test]
fn parse_empty_loop() {
    let expected = Loop {
        body: vec![],
        loc: Some(Location { start: 0, end: 1 }),
    };
    assert_eq!(parse("[]").unwrap(), [expected]);
}
