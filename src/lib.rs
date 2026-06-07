//! `agent-persona`: define and apply personas for LLM agent system prompts.
//!
//! This crate provides a small, dependency-free way to describe an agent's
//! persona — its name, role, tone, instructions, constraints, and few-shot
//! examples — and render it into a deterministic system-prompt string suitable
//! for passing to an LLM.
//!
//! # Example
//!
//! ```rust
//! use agent_persona::PersonaBuilder;
//!
//! let p = PersonaBuilder::new("Alice")
//!     .role("technical writer")
//!     .tone("concise and clear")
//!     .instruction("always use code examples")
//!     .build();
//!
//! let prompt = p.system_prompt();
//! assert!(prompt.contains("Alice"));
//! assert!(prompt.contains("technical writer"));
//! ```

/// A persona definition for an LLM agent.
///
/// Construct one with [`PersonaBuilder`], then call [`Persona::system_prompt`]
/// to render it into a system-prompt string. All fields are public so a
/// `Persona` can also be assembled directly when convenient.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Persona {
    /// The agent's name (e.g. `"Alice"`).
    pub name: String,
    /// An optional role description (e.g. `"technical writer"`).
    pub role: Option<String>,
    /// An optional tone description (e.g. `"concise and clear"`).
    pub tone: Option<String>,
    /// Free-form instructions the agent should follow.
    pub instructions: Vec<String>,
    /// Hard constraints the agent must respect.
    pub constraints: Vec<String>,
    /// Few-shot examples illustrating desired behavior.
    pub examples: Vec<String>,
}

impl Persona {
    /// Build a system-prompt string from this persona.
    ///
    /// The output is deterministic: sections always appear in the order
    /// name/role, tone, instructions, constraints, examples. Empty sections are
    /// omitted entirely (no dangling headers). When a role is present, the
    /// indefinite article (`a`/`an`) is chosen based on the role's first
    /// letter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use agent_persona::PersonaBuilder;
    ///
    /// let p = PersonaBuilder::new("Ada").role("engineer").build();
    /// assert_eq!(p.system_prompt(), "You are Ada, an engineer.");
    /// ```
    pub fn system_prompt(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        // Name and role.
        match &self.role {
            Some(role) => parts.push(format!(
                "You are {}, {} {}.",
                self.name,
                indefinite_article(role),
                role
            )),
            None => parts.push(format!("You are {}.", self.name)),
        }

        if let Some(tone) = &self.tone {
            parts.push(format!("Your tone is {}.", tone));
        }

        push_section(&mut parts, "Instructions:", &self.instructions);
        push_section(&mut parts, "Constraints:", &self.constraints);
        push_section(&mut parts, "Examples:", &self.examples);

        parts.join("\n")
    }
}

/// Append a `header` followed by one `- item` line per entry, but only if
/// `items` is non-empty.
fn push_section(parts: &mut Vec<String>, header: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }
    parts.push(header.to_string());
    for item in items {
        parts.push(format!("- {}", item));
    }
}

/// Choose the indefinite article (`"a"` or `"an"`) for the given word based on
/// its first alphabetic character. This is a simple heuristic on the leading
/// letter, not full phonetic analysis (e.g. it returns `"a"` for `"hour"`).
fn indefinite_article(word: &str) -> &'static str {
    match word
        .chars()
        .find(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
    {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => "an",
        _ => "a",
    }
}

/// Builder for constructing a [`Persona`].
///
/// Every setter takes ownership of `self` and returns it, so calls can be
/// chained fluently and terminated with [`PersonaBuilder::build`].
#[derive(Debug, Clone)]
pub struct PersonaBuilder {
    name: String,
    role: Option<String>,
    tone: Option<String>,
    instructions: Vec<String>,
    constraints: Vec<String>,
    examples: Vec<String>,
}

impl PersonaBuilder {
    /// Start building a persona with the given `name`.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            role: None,
            tone: None,
            instructions: Vec::new(),
            constraints: Vec::new(),
            examples: Vec::new(),
        }
    }

    /// Set the agent's role (overwrites any previously set role).
    pub fn role(mut self, role: &str) -> Self {
        self.role = Some(role.to_string());
        self
    }

    /// Set the agent's tone (overwrites any previously set tone).
    pub fn tone(mut self, tone: &str) -> Self {
        self.tone = Some(tone.to_string());
        self
    }

    /// Append a single instruction. Call multiple times to add several;
    /// order is preserved.
    pub fn instruction(mut self, inst: &str) -> Self {
        self.instructions.push(inst.to_string());
        self
    }

    /// Append a single constraint. Order is preserved.
    pub fn constraint(mut self, c: &str) -> Self {
        self.constraints.push(c.to_string());
        self
    }

    /// Append a single few-shot example. Order is preserved.
    pub fn example(mut self, ex: &str) -> Self {
        self.examples.push(ex.to_string());
        self
    }

    /// Finalize the builder into a [`Persona`].
    pub fn build(self) -> Persona {
        Persona {
            name: self.name,
            role: self.role,
            tone: self.tone,
            instructions: self.instructions,
            constraints: self.constraints,
            examples: self.examples,
        }
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
        let p = PersonaBuilder::new("Bot")
            .instruction("be concise")
            .instruction("use bullets")
            .build();
        let s = p.system_prompt();
        assert!(s.contains("be concise"));
        assert!(s.contains("use bullets"));
    }

    #[test]
    fn constraints_in_prompt() {
        let p = PersonaBuilder::new("Bot")
            .constraint("no profanity")
            .build();
        let s = p.system_prompt();
        assert!(s.contains("no profanity"));
    }

    #[test]
    fn examples_in_prompt() {
        let p = PersonaBuilder::new("Bot")
            .example("Q: hi. A: hello.")
            .build();
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
        assert!(p
            .system_prompt()
            .starts_with("You are Alice, an assistant."));
    }

    #[test]
    fn consonant_role_uses_a() {
        let p = PersonaBuilder::new("Bob").role("teacher").build();
        assert_eq!(p.system_prompt(), "You are Bob, a teacher.");
    }

    #[test]
    fn vowel_role_uses_an() {
        let p = PersonaBuilder::new("Eve").role("editor").build();
        assert_eq!(p.system_prompt(), "You are Eve, an editor.");
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

    #[test]
    fn empty_examples_no_header() {
        let p = PersonaBuilder::new("B").build();
        assert!(!p.system_prompt().contains("Examples:"));
    }

    #[test]
    fn sections_appear_in_canonical_order() {
        let p = PersonaBuilder::new("Z")
            .tone("calm")
            .instruction("do x")
            .constraint("never y")
            .example("e")
            .build();
        let s = p.system_prompt();
        let tone = s.find("Your tone").unwrap();
        let instr = s.find("Instructions:").unwrap();
        let cons = s.find("Constraints:").unwrap();
        let ex = s.find("Examples:").unwrap();
        assert!(tone < instr);
        assert!(instr < cons);
        assert!(cons < ex);
    }

    #[test]
    fn default_persona_is_empty() {
        let p = Persona::default();
        assert_eq!(p.system_prompt(), "You are .");
        assert!(p.role.is_none());
        assert!(p.instructions.is_empty());
    }

    #[test]
    fn persona_equality() {
        let a = PersonaBuilder::new("A").role("r").instruction("i").build();
        let b = PersonaBuilder::new("A").role("r").instruction("i").build();
        assert_eq!(a, b);
        let c = PersonaBuilder::new("A").role("r").build();
        assert_ne!(a, c);
    }

    #[test]
    fn last_role_call_wins() {
        let p = PersonaBuilder::new("A")
            .role("first")
            .role("second")
            .build();
        assert_eq!(p.role.as_deref(), Some("second"));
    }
}
