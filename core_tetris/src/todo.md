- TETRIS correct wallkicks using guidelines
- TESTS make more tetris_grid unit tests
- TESTS make core_tetris integration tests : hold, game over, wallkicks
- DOC correct garbage functions doc
- DOC improve MockRng doc
- TETRIS read tetris guidelines to update code and vocabulary


- NEXTNEXT eventually the user will choose the size of the **Next Queue**. A size of 0 will mean there is NO CircularBuffer, therefore
trying to construct a CircularBuffer of size 0 is a library bug. The ctor can panic for a size of 0.
The functions and the fields make no sense for a size of 0. **Next Queue** could be Option<CircularBuffer> (no panic).
- NEXT reduce the number of panics
- NEXT update panic doc
- NEXT review recent CoreTetrisError changes
- NEXT check error messages
