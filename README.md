## CUB

CUB stands for Command-line Utility for [Bear][bear-app] and offers a simple CLI interface
to the notes, tags, and media stored within your bear writer application.

[bear-app]: https://bear-writer.com

## TODO
- [ ] List notes.
    - [ ] Configure rendering options
      - [x] limit
      - [x] w/ text.
      - [ ] w/ creation/modification date.
    - [x] Filter by note status.
    - [ ] Filter by tag.
- [x] View a single note.
- [ ] View fun stats for all notes/note?


## Installing

    brew install cub-cli


## How to do things

Here is how to do some common things with the `cub` CLI. Also check out
`cub --help` to see all command/options you can use.

### Listing out notes.

    # List _all_ notes.
    > cub ls

    # Limit output to 10 notes
    > cub ls --limit 10
    > cub ls -l 10

    # List notes w/ full-text
    # BEWARE: If you have a lot of very large notes this _will_ output the
    # entirety of the text to the terminal.
    > cub ls --text
    > cub ls -t

### View a note.

    # Notes are prefixed with their ID in the `ls` command. Use that ID
    # here to output the full text.
    > cub show 571


## Building from scratch

CUB is build using the latest stable version of rust-lang and can be built
from scratch using the following command:

    cargo build