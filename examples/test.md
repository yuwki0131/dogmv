# Welcome to dogmv!

This is a **Markdown Viewer** for NixOS/Hyprland.

## Features

- ✅ GitHub Flavored Markdown support
- ✅ Beautiful rendering with CSS
- ✅ Code blocks with syntax highlighting (coming soon)
- ✅ Tables, task lists, and more

## Example Code Blocks

### Rust with Syntax Highlighting

```rust
fn main() {
    println!("Hello, dogmv!");

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();

    println!("Sum: {}", sum);
}
```

### Python

```python
def factorial(n):
    """Calculate factorial recursively"""
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print(factorial(5))  # Output: 120
```

### JavaScript

```javascript
const greeting = (name) => {
    console.log(`Hello, ${name}!`);
};

greeting('dogmv');
```

### Bash

```bash
#!/bin/bash
echo "Building dogmv..."
cargo build --release
echo "Done!"
```

### Plain Code (no highlighting)

```
This is plain code
without any syntax highlighting
```

## Example Table

| Feature | Status |
|---------|--------|
| Markdown rendering | ✅ Done |
| Syntax highlighting | 🚧 In progress |
| File watching | 📋 Planned |

## Task List

- [x] Implement GTK4 window
- [x] Integrate WebView
- [x] Markdown rendering
- [ ] Syntax highlighting
- [ ] File watching
- [ ] Keyboard shortcuts

## Blockquote

> This is a blockquote.
> It can span multiple lines.

## Links and Images

Check out the [dogmv project](https://github.com) for more information.

---

**Built with Rust** 🦀
