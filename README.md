# prompt-organizer

> A simple library to better manage AI prompts in your Rust code.

[<img alt="github" src="https://img.shields.io/badge/github-m1guelpf/prompt-organizer-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/m1guelpf/prompt-organizer)
[<img alt="crates.io" src="https://img.shields.io/crates/v/prompt-organizer.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/prompt-organizer)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-prompt-organizer-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/prompt-organizer)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/m1guelpf/prompt-organizer/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/m1guelpf/prompt-organizer/actions?query=branch%3Amain)

This crate provides a procedural macro for defining AI prompts (or any multi-line string which might include user-provided parameters) in a more readable way.

```bash
cargo add prompt-organizer
```

## Usage

```rust
use prompt_organizer::prompt;

prompt!{my_example_prompt, "
    You are {name}, an AI assistant.
"}

assert_eq!(my_example_prompt("some name"), "You are some name, an AI assistant.");
```

If you need to include quotes in your prompt, you can use string literals:

```rust
use prompt_organizer::prompt;

prompt!{my_example_prompt, r#"
    You are {name}, an AI assistant.

    "this is part of the prompt"
"#}

assert_eq!(my_example_prompt("some name"), "You are some name, an AI assistant.\n\n\"this is part of the prompt\"");
```

The `prompt!` macro will automatically normalize the indentation of your prompt, allowing for nicer formatting in-code.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
