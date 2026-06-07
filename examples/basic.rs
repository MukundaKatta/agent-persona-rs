//! A runnable example: build a persona and print its rendered system prompt.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example basic
//! ```

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
