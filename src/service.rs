#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageId {
    Accueil,
    Arcade,
    Messagerie,
    Infos,
    Gpe,
    Noeud7,
    Aide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavCommand {
    Digit(u8),
    Send,
    Correction,
    Return,
    Summary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    current: PageId,
    pending: Option<u8>,
    history: Vec<PageId>,
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

impl Service {
    pub fn new() -> Self {
        Self {
            current: PageId::Accueil,
            pending: None,
            history: Vec::new(),
        }
    }

    pub fn current_page(&self) -> PageId {
        self.current
    }

    pub fn pending_digit(&self) -> Option<u8> {
        self.pending
    }

    pub fn available_pages() -> &'static [PageId] {
        &[PageId::Accueil]
    }

    pub fn apply(&mut self, _command: NavCommand) {
        // Intentionally empty for the RED TDD commit.
    }
}
