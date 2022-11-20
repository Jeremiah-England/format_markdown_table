# Format Markdown Table

Just a small little CLI that takes a markdown table and formats it to be
evently spaced. My intented use it from inside of Neovim where I can highlight
the rows of the table and pipe them to the `format_markdown_table` executable.
The formatted table is sent to standard out.

## My Setup

Clone this repository and run this in the root directory.

```
cargo install --path .
```

The `format_markdown_table` executable should be in your path now.

Then from inside of vim/neovim you can highlight the rows of the table and type
"!". Then you should be able to type `forma...` and tab to complete the name.
Your table should look nice now!

You can try it out on this table if you would like:

```markdown
| City | Population |
|-|-|
| Anchorage | 300,000 |
| Seattle | 734,000 |
| New York City | 8,500,000 |
```

It should look like this now!

```markdown
| City          | Population |
|---------------|------------|
| Anchorage     | 300,000    |
| Seattle       | 734,000    |
| New York City | 8,500,000  |
```
