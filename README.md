# FreeWrite Sync

A personal utility to move files from my [freewrite](https://getfreewrite.com) to my [obsidian](https://obsidian.md) vault(s).

## Usage
```
./freesync2 <input> <output>
```

## Syntax
Notes need to have a valid title which can be used in the Obsidian Vault, which can be included as:
```
# Title

With a blank line before this content. Missing title, wrong header level, no blank line, or no content will be skipped.
```

## Deploy
```
cargo build --release
```

```
sudo cp target/release/freesync2 /usr/local/bin/
```

## Features
1. Iterates files in the input directory and moves to output directory.
2. Detect markdown titles and change the name in the outdir.
3. Remove reserved characters if they appear in the title.
4. Smart append if the file already exists in the out dir.