# agent-persona

[![CI](https://github.com/MukundaKatta/agent-persona-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/MukundaKatta/agent-persona-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Define and apply **personas** for LLM agent system prompts.

`agent-persona` is a small, **dependency-free** Rust crate for describing an
agent's persona — its name, role, tone, instructions, constraints, and few-shot
examples — and rendering it into a deterministic system-prompt string you can
hand to any LLM.

## Why

When you build LLM agents you usually end up hand-concatenating strings to form
a system prompt. That gets repetitive and error-prone. `agent-persona` gives you
a typed builder and a single, predictable rendering so prompts stay consistent
across agents and over time.

- **Zero dependencies** — nothing pulled into your dependency tree.
- **Deterministic output** — sections always render in the same order, and
  empty sections are omitted (no dangling headers).
- **Fluent builder** — chain setters and call `.build()`.
- **Plain `String` output** — works with any LLM client.

## Install

Add it to your `Cargo.toml`:

```toml
[dependencies]
agent-persona = "0.1"
```

Or with the Cargo CLI:

```sh
cargo add agent-persona
```

## Usage

```rust
use agent_persona::PersonaBuilder;

fn main() {
    let persona = PersonaBuilder::new("Ada")
        .role("senior Rust engineer")
        .tone("precise, friendly, and direct")
        .instruction("explain trade-offs before recommending a solution")
        .instruction("include runnable code where it helps")
        .constraint("never invent crate names or APIs")
        .example("Q: how do I read a file? A: use std::fs::read_to_string")
        .build();

    println!("{}", persona.system_prompt());
}
```

Output:

```text
You are Ada, a senior Rust engineer.
Your tone is precise, friendly, and direct.
Instructions:
- explain trade-offs before recommending a solution
- include runnable code where it helps
Constraints:
- never invent crate names or APIs
Examples:
- Q: how do I read a file? A: use std::fs::read_to_string
```

A runnable version of this lives in [`examples/basic.rs`](examples/basic.rs):

```sh
cargo run --example basic
```

## API

### `PersonaBuilder`

Fluent builder. Every setter takes and returns `self`, so calls can be chained.

| Method | Description |
| --- | --- |
| `PersonaBuilder::new(name: &str)` | Start a builder with the given name. |
| `.role(role: &str)` | Set the role (last call wins). |
| `.tone(tone: &str)` | Set the tone (last call wins). |
| `.instruction(inst: &str)` | Append an instruction (order preserved). |
| `.constraint(c: &str)` | Append a constraint (order preserved). |
| `.example(ex: &str)` | Append a few-shot example (order preserved). |
| `.build()` | Finalize into a [`Persona`]. |

### `Persona`

The rendered model. All fields are public, so you can also construct one
directly without the builder.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | `String` | The agent's name. |
| `role` | `Option<String>` | Optional role description. |
| `tone` | `Option<String>` | Optional tone description. |
| `instructions` | `Vec<String>` | Instructions to follow. |
| `constraints` | `Vec<String>` | Hard constraints to respect. |
| `examples` | `Vec<String>` | Few-shot examples. |

`Persona` derives `Debug`, `Clone`, `PartialEq`, `Eq`, and `Default`.

#### `Persona::system_prompt(&self) -> String`

Renders the persona into a system-prompt string. The output is deterministic:

1. The identity line — `You are <name>.` or `You are <name>, <a/an> <role>.`
   The indefinite article (`a` / `an`) is chosen from the role's first letter.
2. `Your tone is <tone>.` (only if a tone is set)
3. An `Instructions:` block (only if non-empty)
4. A `Constraints:` block (only if non-empty)
5. An `Examples:` block (only if non-empty)

Empty sections are omitted entirely, so you never get a header with no items.

## Development

```sh
cargo build
cargo test            # unit + integration + doc tests
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --example basic
```

## License

Licensed under the [MIT License](LICENSE).
