~~API TetrisPlayer should have 2 rngs, changeable anytime~~
=> the issue with this is that it makes everything generic (RenderTetrisCore) and muddies serialization

0- Separate MockRng from CircularBuffer (make it a newtype ?) : *OK*
Harmonize MockRng and MockGarbageRng (values inside should be precomputed, next function shared as well as
TryRng implementation). New functions should reference the precise usage.

1- CLEAN add_garbage could rely on rng provided by outside of the crate -> great for integration tests (or add_garbage could have a very precise number from outside ?)

2- PERF try tweaking TetrisPlayer serialization so that the tetris grid isn't sent

- BUG the tetrominos should not change the first time the game is started... start the game automatically when clicking on the button
=> tricky change at the moment
- TESTS make core_tetris integration tests
=> not a priority
- CLEAN code getters and make fields private (TetrisPlayer)
- BUG fix : TetrisPlayer should receive the number of garbage lines to add, not the number of completed lines !!
- DOC improve doc
- TESTS make more tetris_grid unit tests
- TETRIS correct srs using guidelines
- TETRIS read tetris guidelines to update code and vocabulary
