

# General purpose

The goal of this project is to provide a **CLI tool** to add any number of components to your project. 
It works for **Leptos** at the moment, but it will be extended to other frameworks in the future.


# Installation


```bash
cargo install ui-cli
# └─> Don't forget to regularly run: `cargo install-update -a` (frequent updates)
```


# Usage
*Step 1: Create a cargo project*
```bash
cargo new my_project
cd my_project
```

*Step 2: Initialisation*
```bash
ui init
```

*Step 3: Add components*
```bash

ui add button
# ui add demo_card demo_button
# └──> Works with any number of components
```

*Step 4: Bolier plate code*
Create `index.html` in project root directory and update `src/main.rs` with following code
_src/main.rs_
```rust
use leptos::prelude::*;

mod components;    // <--- make sure to add this line

use components::ui::button::Button;

fn main() {
    leptos::mount::mount_to_body(move || view! { <App/> });
}

#[component]
fn App() -> impl IntoView {

    let (count, set_count) = signal(0);

    view! {
        <div>
            <p>"Count: "{count}</p>
            <Button on:click=move |_| set_count.update(|count| *count += 1) >"Hit Me"</Button>
        </div>
    }
}
```

Now run it with trunk or any other tool like `$ trunk serve`

# Contributions 💪

It works but it's very messy and there is a lot of room for improvements.

Any contribution is welcome!



# License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
