# CLI Application to visualize Forsyth-Edwards Notation (FEN)

## Usage

__Usage:__`chess_fen_visualizer.exe [OPTIONS] --configuration <CONFIGURATION>`

__Options__:

| Option                              | Description                                           |
|-------------------------------------|-------------------------------------------------------|
| -c, --configuration <CONFIGURATION> | Valid FEN configuration. Row separated by "/"         |
| -o, --output                        | Alternative output file path: Default is ./Output.png |
| -h, --help                          | Print help                                            |
| -V, --version                       | Print version                                         |


Examples:
`cargo run -- -o ./Finalfile.png -c 8/8/8/4p1K1/2k1P3/8/8/8`

or 

`.\chess_fen_visualizer.exe -o ./Finalfile.png -c 8/8/8/4p1K1/2k1P3/8/8/8`