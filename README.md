# clipgraph
A simple CLI to parse, graph, and analyze numerical data pasted to the clipboard.

The stats and graphing logic comes from these crates for now:
[rasciigraph](https://github.com/orhanbalci/rasciigraph)
[histo](https://docs.rs/histo/latest/histo/index.html) 
[linregress](https://docs.rs/linregress/0.5.4/linregress/index.html)
but I may swap them out later as I adjust things.

Usage:

`
clipgraph [OPTIONS]
`

Case-sensitive options:
- "p" Prints the numerical data that could be parsed from the clipboard
- "l" ASCII line graph of the values (in the order presented)
- "h" 10-bucket histogram with exploratory stats (currently supports only positive numbers, and rounds to integers)
- "r" currently a limited OLS for two columns of data with tab delimitation. Y is left, X is right.

Input data from the system clipboard can be delimited by newlines or tabs. Not robust yet but works with e.g. copying a column or a row from a spreadsheet.

The "r" option only works for two columns of tab-delimited data at the moment.

To-dos:
- Work with other delimiters
- add Clap, the CLI is currently very minimal
- error handling
- throw in some other plotting libraries
- build out the linear regression capabilities.
