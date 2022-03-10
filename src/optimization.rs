use crate::parser::*;

use itertools::Itertools;

pub trait Merge<T> {
    fn merge(&self, other: T) -> T;
}

impl Merge<Option<Location>> for Option<Location> {
    fn merge(&self, other: Option<Location>) -> Option<Location> {
        match (*self, other) {
            (Some(self_loc), Some(other_loc)) => {
                let (first_loc, second_loc) = if self_loc.start <= other_loc.start {
                    (self_loc, other_loc)
                } else {
                    (other_loc, self_loc)
                };

                if first_loc.end + 1 >= second_loc.start {
                    Some(Location {
                        start: first_loc.start,
                        end: second_loc.end,
                    })
                } else {
                    Some(other_loc)
                }
            }
            _ => None,
        }
    }
}

trait MapLoopsExt: Iterator<Item = Node> {
    fn map_loops<F>(&mut self, f: F) -> Vec<Node>
    where
        F: Fn(Vec<Node>) -> Vec<Node>,
    {
        self.map(|instr| match instr {
            Node::Loop { body, loc } => Node::Loop { body: f(body), loc },
            other => other,
        })
        .collect()
    }
}

impl<I> MapLoopsExt for I where I: Iterator<Item = Node> {}

/// Sets cell to 0 when [-]
pub fn zero_loop(instructions: Vec<Node>) -> Vec<Node> {
    instructions
        .into_iter()
        .map(|instruction| {
            if let Node::Loop { ref body, loc } = instruction {
                if body.len() == 1 {
                    if let Node::CellShift { amount: -1, .. } = body[0] {
                        return Node::Set { amount: 0, loc };
                    }
                }
            }
            instruction
        })
        .map_loops(zero_loop)
}

/// Merges concurrent cell shifts into 1 shift
pub fn merge_cell_shifts(instructions: Vec<Node>) -> Vec<Node> {
    instructions
        .into_iter()
        .coalesce(|prev_instruction, instruction| {
            if let Node::CellShift {
                amount: prev_amount,
                loc: prev_loc,
            } = prev_instruction
            {
                if let Node::CellShift { amount, loc } = instruction {
                    return Ok(Node::CellShift {
                        amount: amount + prev_amount,
                        loc: prev_loc.merge(loc),
                    });
                }
            }
            Err((prev_instruction, instruction))
        })
        .filter(|instruction| {
            if let Node::CellShift { amount: 0, .. } = *instruction {
                return false;
            }
            true
        })
        .map_loops(merge_cell_shifts)
}

/// Merges concurrent pointer shifts
pub fn merge_pointer_shifts(instructions: Vec<Node>) -> Vec<Node> {
    instructions
        .into_iter()
        .coalesce(|prev_instruction, instruction| {
            if let Node::PointerShift {
                amount: prev_amount,
                loc: prev_loc,
            } = prev_instruction
            {
                if let Node::PointerShift { amount, loc } = instruction {
                    return Ok(Node::PointerShift {
                        amount: amount + prev_amount,
                        loc: prev_loc.merge(loc),
                    });
                }
            }
            Err((prev_instruction, instruction))
        })
        .filter(|instruction| {
            if let Node::PointerShift { amount: 0, .. } = *instruction {
                return false;
            }
            true
        })
        .map_loops(merge_pointer_shifts)
}

#[test]
fn test_merge_cell_shifts() {
    let tree = parse("+++").unwrap();
    let expected = vec![Node::CellShift {
        amount: 3,
        loc: Some(Location { start: 0, end: 2 }),
    }];
    assert_eq!(merge_cell_shifts(tree), expected);
}
