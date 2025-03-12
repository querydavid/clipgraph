# clipgraph
A simple CLI to parse and graph numerical data pasted to the clipboard

Uses the [rasciigraph](https://github.com/orhanbalci/rasciigraph) and [histo](https://docs.rs/histo/latest/histo/index.html) crates.

Usage:

`
clipgraph [OPTIONS]
`

Input data from the system clipboard should be newline-delimited. Not robust yet but works with e.g. copying a column from a spreadsheet.

Case-sensitive options:
- "p" Prints the numerical data that could be parsed from the clipboard
- "l" ASCII line graph of the values (in the order presented)
- "h" 10-bucket histogram with exploratory stats (currently supports only positive numbers, and rounds to integers)

To-dos:
- Work with other delimiters
- add Clap, the CLI is currently very minimal
- error handling
- throw in some other plotting libraries
