use std::io::{IsTerminal, Write};

/// Interaction seam for the onboarding flow so tests can script confirmations.
pub trait Ui: Send {
    fn is_interactive(&self) -> bool;
    /// Ask a yes/no question; returns true only on an explicit yes.
    fn confirm(&mut self, prompt: &str) -> bool;
}

/// Real terminal UI (matches the hand-rolled prompt style in `configure`).
pub struct TtyUi;

impl Ui for TtyUi {
    fn is_interactive(&self) -> bool {
        std::io::stdin().is_terminal()
    }

    fn confirm(&mut self, prompt: &str) -> bool {
        eprint!("{prompt} [y/N] ");
        let _ = std::io::stderr().flush();
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() {
            return false;
        }
        matches!(line.trim().to_lowercase().as_str(), "y" | "yes")
    }
}
