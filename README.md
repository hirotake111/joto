# joto

joto is a simple JWT decoder.

### Installation

```bash
cargo build --release
```

### Usage

You can pass JWT input as an argument:

```bash
joto "eyJ..."
```

Or, you can pass it through pipe:

```bash
cat data.txt | joto
```
