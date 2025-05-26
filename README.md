### SPELL CHECKING USING BLOOM FILTER

This project implements a spell checker using a Bloom filter. The Bloom filter is a space-efficient probabilistic data structure that is used to test whether an element is a member of a set. It can return false positives but never false negatives, making it suitable for applications like spell checking.

### CLI COMMANDS

- `spellcheck --file <file_path> --mode <mode>`: Checks the spelling of words in the specified file.
- `--mode` can either be `bloom` or `db`.
- `--mode bloom`: Uses a Bloom filter for spell checking.
- `--mode db`: Uses a database for spell checking.

### CLONING THE REPOSITORY

To clone the repository, run:

```bash
        git clone https://github.com/manishdotsqlite/spell-checker.git
```
