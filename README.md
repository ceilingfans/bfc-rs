
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
An example --- let's take a English sentence, `I am sleeping.`, and act like the lexer. We can *tokenize* the sentence into something like:
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
No, its not a plant, its just a data structure most programming languages use to grammarticize the stream of tokens.
Lets take math for example. Say we have a simple expression like `x = 1 + 2 * 3`. Once we *tokenize* it we will get something like:
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

Yup, in computer science most trees are upside down, this is probably why they dont make good plants. Alright then, now we have our tree, so the computer finally can understand the expression.

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
`Read` and `Write` represent the ',' and '.' commands repectively
Lastly, `Loop` represents the '[' and ']' commands

**Alright, but what does the 'i8', 'Option\<Location>' and 'Vec\<Node>' do?**
The language we are writing the transpiler in is [Rust](https://www.rust-lang.org/) and those are the data types that are provided to us.

**Okay but that doesn't answer my question, what even is 'i8'?? eight i's??**

The `i8` data type represents an 8-bit signed integer which can hold any integer from **-128 to 127**
The `Option<...>` data type is an enumeration with the values of Some and None, all you need to know is that it is basically an optional value.
The `Vec<...>` data type represents a vector of sometimes, you can think of it as a list.
