# UI CLI

A **CLI tool** to add any number of components to your project, or to start a new project from scratch. 
It works for **Leptos** at the moment, but it will be extended to other frameworks in the future. 
All the components are built using **Tailwind CSS**.



## Installation

```bash
cargo install ui-cli --force
```


## Commands

### 1. Starters (optional, quick start)

If you want to start very easily with all setup for you, run this:

```bash
ui starters # Optional, quick start
```


### 2. Init (existing projects)

If you want add components to an existing project, run this:

```bash
ui init
```

This command will setup everything for you to then add components easily.


### 3. Add

For adding new components, you just need to run this:

```bash
ui add button
# ui add demo_card demo_button
# └──> Works with any number of components
```


## Example in Production

This crate is used in [rust-ui.com](https://www.rust-ui.com) — check it out to see UI CLI in action :)





## Contributions 💪

Still room for improvements.

Any contribution is welcome!



## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
