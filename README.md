
# bfc-rs
Yet another [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) to C transpiler.

### Getting Started
todo

### Build
todo

### Usage
todo

# Documenting the Process
**What on earth is Brainfuck?**

Brainfuck is an esoteric programming language created in 1993 by Urban Müller.  
Notable for its extreme minimalism, the language consists of only eight simple commands, a data pointer and an instruction pointer. While it is fully Turing complete, it is not intended for practical use, but to challenge and amuse programmers. Brainfuck simply requires one to break commands into microscopic steps.

**Okay, now what do we start with?**

In *all* programming languages, to convert human-readable text to something the computer can understand, we need to write a [Lexer](https://en.wikipedia.org/wiki/Lexical_analysis). Now you might be wondering, "what is a lexer, never heard of it". Well let's take a look at it.

A lexer is a program that takes the source code and translates it into a stream of things called [Tokens](https://en.wikipedia.org/wiki/Lexical_analysis#Tokenization). Now you might still ask, "what is a token, this is getting more confusing".

A token is basically something that the computer can use to group your source code into specific chunks.
An example --- let's take an English sentence, `I am sleeping.`, and act like the lexer. We can *tokenize* the sentence into something like:
```
Pronoun {
	literal: "I"
},
Verb {
	literal: "am"
},
Verb {
	literal: "sleeping"
},
Punctuation(Period) {
	literal: "."
}
```
Now you may be wondering, "what does that even mean? what is up with the funky curly braces?". What the diagram shows is how we can group text into specific chunks. In the example we used `I am sleeping.` as the text and *tokenized* it to `[ Pronoun, Verb, Verb, Punctuation ]`. You may be still wonder why do we even need to *tokenize* text --- afterall, can't the computer just translate the text to something we can run? We need to understand that computers have no idea what the concept of grammar and text are, text is stored as a bunch of numbers.

**Alright, I think I know what a Lexer is, now what?**

Once we have our stream of tokens, we need to parse it. "how do you do that??" might be the question in your mind now, so let's answer it.

To *parse* the stream of tokens we need to write a program called the [Parser](https://en.wikipedia.org/wiki/Parsing) (who would have guessed). "what does the parser give you?", well in most cases, it will return an [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree).

The Abstract Syntax Tree. What even is that?? Which country does the tree come from??
No, it's not a plant, it's just a data structure most programming languages use to grammarticize the stream of tokens.
Let's take math for example. Say we have a simple expression like `x = 1 + 2 * 3`. Once we *tokenize* it we will get something like:
```
Variable {
	literal: "x",
	type: number,
},
Operator(Assign) {
	literal: "="
},
Number(Integer) {
	literal: "1",
	value: 1,
},
Operator(Plus) {
	literal: "+"
},
Number(Integer) {
	literal: "2",
	value: 2,
},
Operator(Multiply) {
	literal: "*"
},
Number(Interger) {
	literal: "3",
	value: 3,
}
```
Nice, we have our stream of tokens now. If we evaluate the expression mentally, we can express it as:
```
x = 1 + 2 * 3
x = 1 + 6
x = 7
```
But how do you get the computer to do this? And why did I use this expression specifically?
Firstly, we need to understand the [Precedence](https://en.wikipedia.org/wiki/Order_of_operations) of operations in math, from a young age we should have learned the acronym of BODMAS where we evaluate the brackets first, then the orders (indices and roots), then the divisions, then the multiplications, then the additions and subtractions. This is why I used this expression as the example, it shows how we need to parse the equation before we evaluate it.

**Cool, let me see the Abstract Syntax Tree of the expression now**

Alright then, but before that, let's try to define a grammar on how very basic math works:
```
# BODMAS excluding the brackets
assign-to-variable: <variable> <assign> <expressoin>

expression: <number> OR
			<divide-multiply-expression> OR
			<add-minus-expression>
			
add-minus-expression: <expression> (<add> OR <minus>) <expression>

divide-multiply-expression: <expression> (<divide> OR <multiply>) <expression>

# the basic setup
variable: [a name like "x" or "cats"]
assign: =
divide: /
multiply: *
minus: -
add: + 
number: [self-explanatory]
```

Now this might make no sense to you but hopefully once you see the Abstract Syntax Tree this generates using the example expression we evaluated mentally above it will make more sense.
```
x = 1 + 2 * 3

# tree
assign-to-variable
		|
	-----------
	|    |    ┕ expression
	|    ┕ =	|
	┕x		┕ add-minus-expression
			     |    | 	┕ divide-multiply-expression
			     |	  ┕ +	    |		|       ┕ 3
			     ┕ 1	    |		┕ *
			    		    ┕ 2
```

**Woah, that tree is upside down**

Yup, in computer science most trees are upside down, this is probably why they don't make good plants. Alright then, now we have our tree, so the computer finally can understand the expression.

## Let's apply this to the project
**Tokens**

Brainfuck has only 8 tokens --- this may or may not be the reason I chose to use this language
These tokens are:
* \+ --- Increments the current cell's value by 1
* \- --- Decrements the current cell's value by 1
* \> --- Shifts the cell pointer in the positive direction by 1
* \< --- Shifts the cell pointer in the negative direction by 1
* [ --- Starts a loop body
* ] --- Ends a loop body
* , --- Reads a single [Byte](https://en.wikipedia.org/wiki/Byte) from the standard input
* . --- Writes the byte stored in the current cell to the standard output

**Let's see some code**

```rust
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
```
**Jeez, what the hell is that**

Let's start by ignoring all the `#[derive(...)]` lines. With that said, we can now dive into the code, we first want to look at the `Location` structure, This structure allows us to locate where the token is in the code. Now let's take a look at the cooler data type, the `Node` enumeration. What is it? --- This enumeration represents the different tokens mentioned above.
`CellShift` represents the '+' and '-' commands in Brainfuck,
`PointerShift` represents the '>' and '<' commands,
`Read` and `Write` represent the ',' and '.' commands respectively
Lastly, `Loop` represents the '[' and ']' commands

**Alright, but what does the 'i8', 'Option\<Location>' and 'Vec\<Node>' do?**

The language we are writing the transpiler in is [Rust](https://www.rust-lang.org/) and those are the data types that are provided to us.

**Okay but that doesn't answer my question, what even is 'i8'?? eight i's??**

* The `usize` data type is an architecture based unsigned integer data type, on 32-Bit systems, it is an unsigned 32-bit integer while on 64-bit systems it is an unsigned 64-bit integer
* The `i8` data type represents an 8-bit signed integer which can hold any integer from **-128 to 127**
* The `Option<...>` data type is an enumeration with the values of Some and None, all you need to know is that it is basically an optional value.
* The `Vec<...>` data type represents a vector of sometimes, you can think of it as a list.

**Woah, that's a lot to take in**

It is certainly a lot to take in. Let's confuse you more, now we need to *parse* the tokens we got from our lexer. In this case, we are going to combine the Lexer and Parser as all tokens are single characters and do not need a special parser as we just need to iterate through each character 1 at a time.

**Alright then, show me the code for the parser**
```rust
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
```

**Let's start with the first line:**
```rust
pub fn parse(source: &str) -> Result<Vec<Node>, ParserError> {
```
This line declares the `parse` function and will contain the code that will *parse* a *string*, which is our text. The function will then return a **Vector** of **Node**s or a **ParserError**.

```rust
let mut instructions = vec![];
let mut stack = vec![];
```
These lines create the storage we need to store what we have parsed. `instructions` will hold the potential return value of our parser while `stack` will hold the instructions of any loop it encounters, it also serves a secondary function of checking if we have properly used the `[]` syntax for loops.

```rust
for (index, c) in source.chars().enumerate() {
```
This line is quite crucial to our parser, it allows it to iterate through each character in our source.

```rust
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
```
These lines are the meat and bones of our parser, it matches the character with one of the Brainfuck commands. All the commands except the `[` and `]` are quite uninteresting. In that case, let's dive into what these do.
```rust
'[' => {
    stack.push((instructions, index));
    instructions = vec![];
}
```
When we open a loop, we want to store the previous instructions into a temporary storage which is our stack. We then use the instructions vector to store the loop's body.

```rust
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
```
This large chunk of code *pops* an item off the stack, if it is valid, we use the instructions stored on the stack as the previous values and the values in the instructions stack as the loop body.

The very last piece of code in the function checks if we closed all our bracket pairs.

