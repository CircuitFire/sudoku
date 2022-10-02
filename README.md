# sudoku
A program designed to help the user solve sudoku puzzles.

The main purpose of this program is to help its user solve the puzzles themselves and not to just auto solve them for you (although it can do that as well).

- As you fill out the puzzle it will auto mark the other nodes their current posibilities, and will also mark any conflicting answers.
- At any point you can increase a guess level and make changes, then if you are unhappy with the changes you can decrease the guess level and revert all changes made at that level.
- You can have the program check if the puzzle is solvable in is current state.
- It has different levels of auto solvers. Rangeing from solving nodes that only have one solution, up to auto solveing the whole puzzle.
- There is also an internal help menu.

## Installation
There is an executible for Windows in the releases section. There is only one file just download the .exe and run it, although windows will most likely complain about it.
So on the "Windows protected your PC" window click "more info" and then the "run anyway" button.

You can also compile it yourself. You will need the Rust lang installed, and then just clone the repo and use cargo build. I recomend running the program in its own terminal window insted of one build into anather program like VS codes terminal.

