# agent-persona

A tiny, dependency-free Rust library for defining **personas** and turning them into
ready-to-use **system prompts** for LLM agents.

You describe an agent's name, role, tone, instructions, constraints, and few-shot
examples through a fluent builder, then call `system_prompt()` to produce a single,
well-structured prompt string you can hand to any LLM.

## Features

- Fluent `PersonaBuilder` API for composing personas piece by piece.
- Deterministic `system_prompt()` rendering with stable section ordering
  (role/name, tone, instructions, constraints, examples).
- Sections are omitted entirely when empty, so prompts stay clean.
- Zero runtime dependencies.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
agent-persona = "0.1"
```

Or with cargo:

```sh
cargo add agent-persona
```

## Usage

```rust
use agent_persona::{Persona, PersonaBuilder};

let persona = PersonaBuilder::new("Alice")
    .role("technical writer")
    .tone("concise and clear")
    .instruction("always use code examples")
    .constraint("no marketing language")
    .example("Q: how do I install? A: run `cargo add agent-persona`.")
    .build();

let prompt = persona.system_prompt();
println!("{prompt}");
```

This produces a system prompt similar to:

```text
You are Alice, a technical writer.
Your tone is concise and clear.
Instructions:
- always use code examples
Constraints:
- no marketing language
Examples:
- Q: how do I install? A: run `cargo add agent-persona`.
```

A minimal persona only needs a name:

```rust
use agent_persona::PersonaBuilder;

let prompt = PersonaBuilder::new("Bot").build().system_prompt();
assert_eq!(prompt, "You are Bot.");
```

## API overview

- `PersonaBuilder::new(name)` — start a new persona.
- `.role(&str)` / `.tone(&str)` — set the optional role and tone.
- `.instruction(&str)` / `.constraint(&str)` / `.example(&str)` — append entries
  (call multiple times to add more).
- `.build()` — produce a `Persona`.
- `Persona::system_prompt()` — render the persona to a prompt string.

`Persona` derives `Debug` and `Clone`, and exposes its fields publicly.

## Development

```sh
cargo build
cargo test
```

## Tech stack

- Language: Rust (edition 2021)
- Dependencies: none

## License

Licensed under the MIT License.
