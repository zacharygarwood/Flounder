# Flounder

![Flounder](./img/flounder.png)

## Overview
Flounder is a [UCI](https://www.chessprogramming.org/UCI) compatible chess engine written in Rust. Despite its unassuming name, Flounder has grown into a serious competitor (~1800 ELO), consistently outperforming its own creator. If you are up to the test, you can challenge it on Lichess [@FlounderBot](https://lichess.org/@/FlounderBot)! Just a heads up, Flounder may be busy competing against other bots or offline.

If you want to learn more about Flounder, you can do so on my [website](https://zacharygarwood.com/projects/flounder)!

## Installation
To build Flounder all you need to do is clone this repository and build a release. This will make an executable called `flounder.exe` in `target/release/`.
```
$ cargo build --release
```

## Usage
Flounder implements the following UCI commands: `uci`, `isready`, `ucinewgame`, `position`, `go`, and `quit`. More about the usage of these commands can be found here [UCI Protocol](https://backscattering.de/chess/uci/).

Below is an example use case of running Flounder. First, it sets the board to the starting position with the move e2e4 played. The `go` command is then used to get the best move for the current player, in this case black, and `bestmove b8c6` is outputted by Flounder. Lastly, `quit` is used to exit out of the program.
```
$ ./flounder.exe
$ position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4
$ go
$ bestmove b8c6
$ quit
```

## Board Representation
- [Bitboards](https://www.chessprogramming.org/Bitboards)

## Move Generation
- [Magic Bitboards](https://www.chessprogramming.org/Magic_Bitboards)

## Search
- [Negamax with Alpha-Beta Pruning](https://www.chessprogramming.org/Alpha-Beta)
- [Quiescence Search](https://www.chessprogramming.org/Quiescence_Search)
- [Iterative Deepening](https://www.chessprogramming.org/Iterative_Deepening)

## Move ordering
- [PV-Move](https://www.chessprogramming.org/PV-Move)
- [Hash Move](https://www.chessprogramming.org/Hash_Move)
- [MVV-LVA (Most Valuable Victim - Least Valuable Aggressor)](https://www.chessprogramming.org/MVV-LVA)

## Evaluation
- [Material Counting](https://www.chessprogramming.org/Material) 
- [Piece-Square Tables](https://www.chessprogramming.org/Piece-Square_Tables) 

## Other
- [Transposition Table](https://www.chessprogramming.org/Transposition_Table) 
- [Zobrist Hashing](https://www.chessprogramming.org/Zobrist_Hashing) 

## Future Work
- [Principal Variation Search](https://www.chessprogramming.org/Principal_Variation_Search) 
- [Killer Move Heuristic](https://www.chessprogramming.org/Killer_Move) 
- [Null Move Pruning](https://www.chessprogramming.org/Null_Move_Pruning) 
- [Late Move Reductions](https://www.chessprogramming.org/Null_Move_Pruning) 
- [Time Management](https://www.chessprogramming.org/Time_Management) 
