/*!
agent-persona: define and apply personas for LLM agent system prompts.

```rust
use agent_persona::{Persona, PersonaBuilder};

let p = PersonaBuilder::new("Alice")
    .role("technical writer")
    .tone("concise and clear")
    .instruction("always use code examples")
    .build();
let prompt = p.system_prompt();
assert!(prompt.contains("Alice"));
assert!(prompt.contains("technical writer"));
```
*/

/// A persona definition for an LLM agent.
#[derive(Debug, Clone)]
pub struct Persona {
    pub name: String,
    pub role: Option<String>,
    pub tone: Option<String>,
    pub instructions: Vec<String>,
    pub constraints: Vec<String>,
    pub examples: Vec<String>,
}

impl Persona {
    /// Build a system prompt string from this persona.
    pub fn system_prompt(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        // Name and role
        match (&self.role, &self.name) {
            (Some(role), name) => parts.push(format!("You are {}, a {}.", name, role)),
            (None, name) => parts.push(format!("You are {}.", name)),
        }

        if let Some(tone) = &self.tone {
            parts.push(format!("Your tone is {}.", tone));
        }

        if !self.instructions.is_empty() {
            parts.push("Instructions:".to_string());
            for inst in &self.instructions {
                parts.push(format!("- {}", inst));
            }
        }

        if !self.constraints.is_empty() {
            parts.push("Constraints:".to_string());
            for c in &self.constraints {
                parts.push(format!("- {}", c));
            }
        }

        if !self.examples.is_empty() {
            parts.push("Examples:".to_string());
            for ex in &self.examples {
                parts.push(format!("- {}", ex));
            }
        }

        parts.join("\n")
    }
}

/// Builder for constructing a Persona.
pub struct PersonaBuilder {
    name: String,
    role: Option<String>,
    tone: Option<String>,
    instructions: Vec<String>,
    constraints: Vec<String>,
    examples: Vec<String>,
}

impl PersonaBuilder {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), role: None, tone: None, instructions: Vec::new(), constraints: Vec::new(), examples: Vec::new() }
    }

    pub fn role(mut self, role: &str) -> Self { self.role = Some(role.to_string()); self }
    pub fn tone(mut self, tone: &str) -> Self { self.tone = Some(tone.to_string()); self }
    pub fn instruction(mut self, inst: &str) -> Self { self.instructions.push(inst.to_string()); self }
    pub fn constraint(mut self, c: &str) -> Self { self.constraints.push(c.to_string()); self }
    pub fn example(mut self, ex: &str) -> Self { self.examples.push(ex.to_string()); self }

    pub fn build(self) -> Persona {
        Persona { name: self.name, role: self.role, tone: self.tone, instructions: self.instructions, constraints: self.constraints, examples: self.examples }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_persona() {
        let p = PersonaBuilder::new("Bot").build();
        let s = p.system_prompt();
        assert!(s.contains("Bot"));
    }

    #[test]
    fn name_and_role() {
        let p = PersonaBuilder::new("Alice").role("researcher").build();
        let s = p.system_prompt();
        assert!(s.contains("Alice"));
        assert!(s.contains("researcher"));
    }

    #[test]
    fn tone_in_prompt() {
        let p = PersonaBuilder::new("Bot").tone("friendly").build();
        let s = p.system_prompt();
        assert!(s.contains("friendly"));
    }

    #[test]
    fn instructions_in_prompt() {
        let p = PersonaBuilder::new("Bot").instruction("be concise").instruction("use bullets").build();
        let s = p.system_prompt();
        assert!(s.contains("be concise"));
        assert!(s.contains("use bullets"));
    }

    #[test]
    fn constraints_in_prompt() {
        let p = PersonaBuilder::new("Bot").constraint("no profanity").build();
        let s = p.system_prompt();
        assert!(s.contains("no profanity"));
    }

    #[test]
    fn examples_in_prompt() {
        let p = PersonaBuilder::new("Bot").example("Q: hi. A: hello.").build();
        let s = p.system_prompt();
        assert!(s.contains("Q: hi."));
    }

    #[test]
    fn full_persona() {
        let p = PersonaBuilder::new("Alice")
            .role("assistant")
            .tone("concise")
            .instruction("use examples")
            .constraint("no HTML")
            .example("example here")
            .build();
        let s = p.system_prompt();
        assert!(s.contains("Alice"));
        assert!(s.contains("assistant"));
        assert!(s.contains("concise"));
        assert!(s.contains("use examples"));
        assert!(s.contains("no HTML"));
        assert!(s.contains("example here"));
    }

    #[test]
    fn no_role_uses_simple_format() {
        let p = PersonaBuilder::new("Bot").build();
        assert!(p.system_prompt().starts_with("You are Bot."));
    }

    #[test]
    fn with_role_uses_role_format() {
        let p = PersonaBuilder::new("Alice").role("assistant").build();
        assert!(p.system_prompt().starts_with("You are Alice, a assistant."));
    }

    #[test]
    fn multiple_instructions_ordered() {
        let p = PersonaBuilder::new("B")
            .instruction("first")
            .instruction("second")
            .build();
        let s = p.system_prompt();
        assert!(s.find("first").unwrap() < s.find("second").unwrap());
    }

    #[test]
    fn clone_is_independent() {
        let p = PersonaBuilder::new("A").role("r").build();
        let p2 = p.clone();
        assert_eq!(p.name, p2.name);
    }

    #[test]
    fn empty_instructions_no_header() {
        let p = PersonaBuilder::new("B").build();
        assert!(!p.system_prompt().contains("Instructions:"));
    }

    #[test]
    fn empty_constraints_no_header() {
        let p = PersonaBuilder::new("B").build();
        assert!(!p.system_prompt().contains("Constraints:"));
    }
}
