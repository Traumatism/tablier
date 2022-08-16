# tablier

### A little Rust library for creating panels

### Example

```rust
use panel::{Panel, PanelBox};

fn main() {
    println!(
        "{}",
        Panel {
            content: "hello world",
            panel_box: PanelBox::ascii(),
        }
        .render()
    );

    println!(
        "{}",
        Panel {
            content: "hello world\nmy name is victor\ni'm 16",
            panel_box: PanelBox::ascii(),
        }
        .render()
    );
}
```