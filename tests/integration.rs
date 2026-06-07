//! Integration tests exercising the public API as an external consumer would.

use agent_persona::{Persona, PersonaBuilder};

#[test]
fn renders_full_prompt_end_to_end() {
    let persona = PersonaBuilder::new("Ada")
        .role("engineer")
        .tone("precise and direct")
        .instruction("explain your reasoning")
        .instruction("prefer small, testable steps")
        .constraint("never invent APIs")
        .example("Q: what is 2+2? A: 4")
        .build();

    let prompt = persona.system_prompt();

    let expected = "\
You are Ada, an engineer.
Your tone is precise and direct.
Instructions:
- explain your reasoning
- prefer small, testable steps
Constraints:
- never invent APIs
Examples:
- Q: what is 2+2? A: 4";

    assert_eq!(prompt, expected);
}

#[test]
fn minimal_persona_only_has_identity_line() {
    let persona = PersonaBuilder::new("Bot").build();
    assert_eq!(persona.system_prompt(), "You are Bot.");
}

#[test]
fn article_is_chosen_from_role_initial() {
    assert_eq!(
        PersonaBuilder::new("X")
            .role("assistant")
            .build()
            .system_prompt(),
        "You are X, an assistant."
    );
    assert_eq!(
        PersonaBuilder::new("X")
            .role("researcher")
            .build()
            .system_prompt(),
        "You are X, a researcher."
    );
}

#[test]
fn struct_can_be_built_directly() {
    let persona = Persona {
        name: "Direct".to_string(),
        role: Some("operator".to_string()),
        tone: None,
        instructions: vec!["stay on task".to_string()],
        constraints: Vec::new(),
        examples: Vec::new(),
    };

    let prompt = persona.system_prompt();
    assert!(prompt.starts_with("You are Direct, an operator."));
    assert!(prompt.contains("- stay on task"));
}

#[test]
fn builder_is_reusable_via_clone() {
    let base = PersonaBuilder::new("Base").tone("neutral");
    let a = base.clone().role("writer").build();
    let b = base.role("reviewer").build();

    assert!(a.system_prompt().contains("a writer"));
    assert!(b.system_prompt().contains("a reviewer"));
    assert!(a.system_prompt().contains("neutral"));
    assert!(b.system_prompt().contains("neutral"));
}
