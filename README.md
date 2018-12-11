# unix-bitwise
## Unix command-line utilities for bitwise operations on streams

### What's this?
These are some little Rust programs you can use to do bitwise operations on streams of text.
For instance, if you want to XOR a file with another, this is as simple as:  
`xor file_a.bin file_b.bin > output.bin`  
Ever wanted a `/dev/one` to complement `/dev/zero?` Well, now you can get that functionality
by using  
`<(not /dev/zero)`  
in your commands.  

Is this something you've always wanted? No? Understandable. But here it is anyway.  

### How to install
You can just go through and run `rustc` on each one in turn, if you like. They have no library 
dependencies beyond Rust stdlib. If you don't like, you can run my little shell script to do that for 
you, and the resulting binaries will all be collected in a `bin` subdirectory of this folder. Neat!  
Once you have built the binaries, you can drop them into `/usr/local/bin` or wherever you
want to make them runnable from the command line.


### Usage
Generally speaking, the programs accept files or file-like objects (e.g. pipes) as arguments.
Unary operators (like NOT) will only accept one input at a time, while binary operators (like XOR)
will accept any number of inputs.
All of them can also accept a stream from stdin in place of a file when the `-` argument is used, e.g.  
`cat file.bin | not -`  
Binary operators can be used with any number of inputs at once, in which case the operation
will simply be repeated using the result of the previous operation and the next file until 
all files are visited.  
Binary operators whose inputs are different lengths will run until the end of the first argument
is reached. If the first argument is longer than the other arguments, then the program will cease
drawing data from the other files as their ends are reached. For example, if file A contains 10 
bytes, file B contains 7 bytes, and file A is the first argument, then the output of AND will
consist of file B ANDed with the first 7 bytes of file A, followed by the three remaining bytes of
file A unchanged from their original values.  
Running a binary operator with only one input stream will always result in that stream being 
either returned unchanged (for XOR, OR, AND) or returned inverted (for XNOR, NOR, NAND).


### But what is this useful for?
I have no idea. The usefulness of these programs is up to you. It's possible they'll have some
usefulness in certain shell scripts or weird hacks, but I have no specific applications in mind.
This simply occurred to me as something that *ought to exist*, but I'm not actually sure why.
