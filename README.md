# todo-rs

Tasks CLI app for terminal.

## Warning

Note this is only tested in Linux and may not work properly on other platforms.

## Install

Requirements:

- [Cargo (rust)](https://www.rust-lang.org/)

Run the follwing command:

```
cargo install --git https://github.com/Brian3647/todo-rs.git
```

## Usage

```
todo (version)

USAGE:
    todo <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    all       Show the full list of tasks
    delete    Delete a task specified by id
    done      Mark a task as done
    help      Prints this message or the help of the given subcommand(s)
    info      Get full info of a task specified by id
    new       Create a new task by prompting the information
    undone    Mark a task as NOT done
```

### Example usage

![(You can see the full image in .github/assets/usage.png)](.github/assets/usage.png)

_This picture shows the integrated terminal of VSCode with Github dark dimmed theme, shell is zsh and uses [starship](https://starship.rs/) theme_
