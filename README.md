# ttt

A text-to-table CLI tool.

Reads a plain text based table and formats the table.
Can format tables as CSV, markdown, HTML, SQL DDL, or JSON.
Implements various column delimiter options, double-quoted strings, and optional column headings.

Work in Progress. Mostly a Rust learning exercise.

## Example use

```bash
# get help
ttt -h

# parse an input text file and convert to markdown
ttt -f md -i text_input.txt -o output.md --has-header --contiguous-delimiters
```
