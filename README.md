![build status](https://github.com/pereferrera/chess_position_db/actions/workflows/test.yml/badge.svg)

Building a database of chess positions // played moves while learning Rust at the same time.
The database associates a FEN key with a set of movement statistics: movement played, times played, win rate for white, win rate for black, etc.
This database can then be used as an opening book or for other purposes e.g. to enrich a chess engine.

## Database format expected

Text, PGN of the game. I am note sure now but I think I am using some of the databases [here](https://rebel13.nl/download/data.html).
See the example test database that is parsed by the unit tests [here](chess_position_db/test_dataset.pgn).


