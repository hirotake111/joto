# jot

jot is a simple JWT decoder.

### Installation

```bash
cargo build --release
```

### Usage

You can pass JWT input as an argument:

```bash
jot "eyJ..."
```

Or, you can pass it through pipe:

```bash
cat data.txt | jot
```
