Based on : https://esolangs.org/wiki/Brainfuck


Basic Brainfuck rules apply.

- Cells are 8 bit with wrapping.
    - It would be nice if this was configurable with 16, 32 or 64 bit cells.
- Tape is positive only default 30_000 cells.
- Input/Output as utf-8
- EOF as 0. 
    - Would be nice as configurable between
    1) EOF as 0
    2) EOF as unchanged
    3) EOF as unchanged with error in next cell

# Extensions
Brainfuck cant really do anything by only reading and writing to stdio.
Do fix this i want to add a new command that executes some system command like opening a file for example.  
`#` and `!` are commonly used already.
I want to stick with a symbol to match the other characters so maybe  pipe `|` is a good choice.

`|` looks at the value of the current cell and chooses to run the corresponding system command. The system command may use multiple cells to the right of the pointer as input and place its output into multiple cells to the right of the pointer.


