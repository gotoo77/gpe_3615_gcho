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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetailId {
    ArcadeScores,
    PixelMaze,
    Mailbox,
    ChatRooms,
    News,
    ServicePublic,
    Alerts,
    GpeProjects,
    Node7File,
    Node7Witnesses,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavCommand {
    Digit(u8),
    Next,
    Previous,
    Send,
    Correction,
    Return,
    Summary,
    Guide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Page(PageId),
    Detail(DetailId),
    Notice(&'static str),
    Summary,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    pub key: u8,
    pub label: &'static str,
    target: Target,
}

const ACCUEIL: &[Entry] = &[
    Entry {
        key: 1,
        label: "ARCADE       JEUX ET DEMOS",
        target: Target::Page(PageId::Arcade),
    },
    Entry {
        key: 2,
        label: "MESSAGERIE   DIALOGUER, RENCONTRER",
        target: Target::Page(PageId::Messagerie),
    },
    Entry {
        key: 3,
        label: "INFOS        ACTUALITES ET COMMUNIQUES",
        target: Target::Page(PageId::Infos),
    },
    Entry {
        key: 4,
        label: "GPE          PROJETS ET LABORATOIRE",
        target: Target::Page(PageId::Gpe),
    },
    Entry {
        key: 5,
        label: "SECRETS      ACCES RESTREINT",
        target: Target::Page(PageId::Noeud7),
    },
    Entry {
        key: 6,
        label: "AIDE",
        target: Target::Page(PageId::Aide),
    },
    Entry {
        key: 0,
        label: "QUITTER",
        target: Target::Quit,
    },
];

const ARCADE: &[Entry] = &[
    Entry {
        key: 1,
        label: "CLASSEMENT GCHO",
        target: Target::Detail(DetailId::ArcadeScores),
    },
    Entry {
        key: 2,
        label: "PIXEL MAZE",
        target: Target::Detail(DetailId::PixelMaze),
    },
    Entry {
        key: 3,
        label: "LE PENDU",
        target: Target::Notice("LE MOT ETAIT: MODEM. VOUS AVEZ PERDU."),
    },
    Entry {
        key: 4,
        label: "DEMONSTRATIONS",
        target: Target::Notice("DEMO GRAPHIQUE: 40 COLONNES. IMPRESSIONNANT."),
    },
    Entry {
        key: 5,
        label: "TELECHARGEMENTS",
        target: Target::Notice("DEBIT ESTIME: 1200 BAUD. BON COURAGE."),
    },
];

const MESSAGERIE: &[Entry] = &[
    Entry {
        key: 1,
        label: "MA BOITE",
        target: Target::Detail(DetailId::Mailbox),
    },
    Entry {
        key: 2,
        label: "SALONS",
        target: Target::Detail(DetailId::ChatRooms),
    },
    Entry {
        key: 3,
        label: "RENCONTRES",
        target: Target::Notice("COMPATIBILITE TELEMATIQUE: INDETERMINEE."),
    },
    Entry {
        key: 4,
        label: "PETITES ANNONCES",
        target: Target::Notice("VENDS MODEM PEU SERVI. CAUSE: INTERNET."),
    },
    Entry {
        key: 5,
        label: "CARNET D'ADRESSES",
        target: Target::Notice("3615 GCHO EST DEJA DANS VOS FAVORIS."),
    },
];

const INFOS: &[Entry] = &[
    Entry {
        key: 1,
        label: "BREVES",
        target: Target::Detail(DetailId::News),
    },
    Entry {
        key: 2,
        label: "SERVICE PUBLIC",
        target: Target::Detail(DetailId::ServicePublic),
    },
    Entry {
        key: 3,
        label: "ALERTES",
        target: Target::Detail(DetailId::Alerts),
    },
    Entry {
        key: 4,
        label: "CONSEILS PRATIQUES",
        target: Target::Notice("CONSEIL: LISEZ LES CONSEILS AVANT DE LES SUIVRE."),
    },
    Entry {
        key: 5,
        label: "COMMUNIQUES",
        target: Target::Notice("AUCUN COMMUNIQUE NE NECESSITE DE COMMUNIQUER."),
    },
    Entry {
        key: 6,
        label: "RECTIFICATIFS",
        target: Target::Notice("LE RECTIFICATIF PRECEDENT EST RECTIFIE."),
    },
];

const GPE: &[Entry] = &[
    Entry {
        key: 1,
        label: "MOTEUR PIXEL",
        target: Target::Notice("GPE: PETIT MOTEUR, GRAND ECRAN CATHODIQUE MENTAL."),
    },
    Entry {
        key: 2,
        label: "LABORATOIRE",
        target: Target::Notice("EXPERIENCE EN COURS. NE TOUCHEZ PAS AU FRAMEBUFFER."),
    },
    Entry {
        key: 3,
        label: "PROJETS EN LIGNE",
        target: Target::Detail(DetailId::GpeProjects),
    },
];

const NOEUD7: &[Entry] = &[
    Entry {
        key: 1,
        label: "LE FICHIER",
        target: Target::Detail(DetailId::Node7File),
    },
    Entry {
        key: 2,
        label: "LES TEMOINS",
        target: Target::Detail(DetailId::Node7Witnesses),
    },
    Entry {
        key: 3,
        label: "CARTES",
        target: Target::Notice("AUCUNE CARTE NE MONTRE CET ENDROIT."),
    },
    Entry {
        key: 4,
        label: "EXPERIENCES",
        target: Target::Notice("EXPERIENCE 3615: SUJET TOUJOURS CONNECTE."),
    },
    Entry {
        key: 5,
        label: "CONTACT",
        target: Target::Notice("LE CONTACT VOUS A DEJA CONTACTE."),
    },
    Entry {
        key: 6,
        label: "REVENIR",
        target: Target::Summary,
    },
];

const AIDE: &[Entry] = &[Entry {
    key: 1,
    label: "TEST DU TERMINAL",
    target: Target::Notice("CLAVIER RECU. TERMINAL CONSIDERE COMME DOCILE."),
}];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    current: PageId,
    detail: Option<DetailId>,
    pending: Option<u8>,
    history: Vec<PageId>,
    notice: Option<&'static str>,
    exit_requested: bool,
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
            detail: None,
            pending: None,
            history: Vec::new(),
            notice: None,
            exit_requested: false,
        }
    }

    pub fn current_page(&self) -> PageId {
        self.current
    }

    pub fn current_detail(&self) -> Option<DetailId> {
        self.detail
    }

    pub fn pending_digit(&self) -> Option<u8> {
        self.pending
    }

    pub fn notice(&self) -> Option<&'static str> {
        self.notice
    }

    pub fn available_pages() -> &'static [PageId] {
        &[
            PageId::Accueil,
            PageId::Arcade,
            PageId::Messagerie,
            PageId::Infos,
            PageId::Gpe,
            PageId::Noeud7,
            PageId::Aide,
        ]
    }

    pub fn entries(&self) -> &'static [Entry] {
        match self.current {
            PageId::Accueil => ACCUEIL,
            PageId::Arcade => ARCADE,
            PageId::Messagerie => MESSAGERIE,
            PageId::Infos => INFOS,
            PageId::Gpe => GPE,
            PageId::Noeud7 => NOEUD7,
            PageId::Aide => AIDE,
        }
    }

    pub fn take_exit_requested(&mut self) -> bool {
        std::mem::take(&mut self.exit_requested)
    }

    pub fn apply(&mut self, command: NavCommand) {
        match command {
            NavCommand::Digit(digit) => {
                self.notice = None;
                if self.detail.is_some() {
                    self.pending = None;
                    self.notice = Some("UTILISEZ RETOUR POUR QUITTER CET ECRAN.");
                    return;
                }
                self.pending = self
                    .entries()
                    .iter()
                    .any(|entry| entry.key == digit)
                    .then_some(digit);
                if self.pending.is_none() {
                    self.notice = Some("CHOIX INVALIDE. LE TERMINAL VOUS JUGE SILENCIEUSEMENT.");
                }
            }
            NavCommand::Next => self.move_selection(1),
            NavCommand::Previous => self.move_selection(-1),
            NavCommand::Send => {
                if self.detail.is_some() {
                    self.notice = Some("AUCUNE COMMANDE A ENVOYER SUR CET ECRAN.");
                    return;
                }
                let Some(digit) = self.pending.take() else {
                    self.notice = Some("SAISISSEZ UN NUMERO AVANT ENVOI.");
                    return;
                };
                let Some(target) = self
                    .entries()
                    .iter()
                    .find(|entry| entry.key == digit)
                    .map(|entry| entry.target)
                else {
                    self.notice = Some("CHOIX INVALIDE.");
                    return;
                };
                self.follow(target);
            }
            NavCommand::Correction => {
                self.pending = None;
                self.notice = None;
            }
            NavCommand::Return => {
                self.pending = None;
                self.notice = None;
                if self.detail.take().is_some() {
                    return;
                }
                self.current = self.history.pop().unwrap_or(PageId::Accueil);
            }
            NavCommand::Summary => self.go_summary(),
            NavCommand::Guide => {
                self.pending = None;
                self.notice = None;
                self.detail = None;
                self.follow(Target::Page(PageId::Aide));
            }
        }
    }

    fn move_selection(&mut self, direction: i32) {
        self.notice = None;
        if self.detail.is_some() {
            return;
        }

        let entries = self.entries();
        if entries.is_empty() {
            self.pending = None;
            return;
        }

        let current_index = self
            .pending
            .and_then(|key| entries.iter().position(|entry| entry.key == key));
        let next_index = match (current_index, direction.is_positive()) {
            (None, true) => 0,
            (None, false) => entries.len() - 1,
            (Some(index), true) => (index + 1) % entries.len(),
            (Some(0), false) => entries.len() - 1,
            (Some(index), false) => index - 1,
        };
        self.pending = Some(entries[next_index].key);
    }

    fn follow(&mut self, target: Target) {
        self.notice = None;
        match target {
            Target::Page(page) => {
                self.detail = None;
                if page != self.current {
                    self.history.push(self.current);
                    self.current = page;
                }
            }
            Target::Detail(detail) => self.detail = Some(detail),
            Target::Notice(message) => self.notice = Some(message),
            Target::Summary => self.go_summary(),
            Target::Quit => self.exit_requested = true,
        }
    }

    fn go_summary(&mut self) {
        self.pending = None;
        self.notice = None;
        self.detail = None;
        self.history.clear();
        self.current = PageId::Accueil;
    }
}
