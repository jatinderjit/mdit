# mdit

`mdit` is a CLI tool that converts documents (Word, PowerPoint, Excel, OpenDocument, RTF, EPUB, CSV, and PDF) into clean GitHub-Flavored Markdown.

There is no custom document parsing or conversion logic here: [`anydoc`](https://crates.io/crates/anydoc) does the interesting work.
This project only accepts file paths, and saves Anydoc's Markdown output.

## Installation

```
cargo install --git https://github.com/jatinderjit/mdit
````

## Usage

```sh
mdit path/to/document.pdf  # outputs path/to/document.md
mdit path/to/document.pdf output.md  # outputs to path
```

## Features

(Copied from [firecrawl/anydoc](https://github.com/firecrawl/anydoc))

Preserves document structure: Headings with anchors, bold/italic/strikethrough, inline
code and code blocks, links and internal cross-references, bulleted/numbered/nested/task
lists with the source's own numbering, tables with merged cells and header rows, block
quotes, footnotes and endnotes, and speaker notes.

**Supported formats**

| Format           | Extensions                                                 |
| ---------------- | ---------------------------------------------------------- |
| Word             | `.doc`, `.docx`, `.docm`                                   |
| PowerPoint       | `.ppt`, `.pps`, `.pot`, `.pptx`, `.pptm`, `.ppsx`, `.ppsm` |
| Excel            | `.xls`, `.xlsx`, `.xlsm`, `.xlsb`                          |
| OpenDocument     | `.odt`, `.ods`, `.odp`                                     |
| Rich Text Format | `.rtf`                                                     |
| EPUB             | `.epub`                                                    |
| CSV              | `.csv`                                                     |
| PDF              | `.pdf`                                                     |

## License

[Unlicense](LICENSE)
