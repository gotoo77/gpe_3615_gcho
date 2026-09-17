#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageId {
    Accueil,
    Services,
    Arcade,
    Messagerie,
    Infos,
    Gpe,
    Noeud7,
    Aide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetailId {
    RoyalCommunique,
    RoyalHeatAlert,
    ArcadeScores,
    PixelMaze,
    Mailbox,
    ChatRooms,
    Rencontres,
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
    Reconnect,
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
        label: "COMMUNIQUE ROYAL",
        target: Target::Detail(DetailId::RoyalCommunique),
    },
    Entry {
        key: 2,
        label: "ALERTE CANICULE",
        target: Target::Detail(DetailId::RoyalHeatAlert),
    },
    Entry {
        key: 3,
        label: "MESSAGES DU JOUR",
        target: Target::Detail(DetailId::News),
    },
    Entry {
        key: 4,
        label: "PORTRAITS PIXELISES",
        target: Target::Notice("CABINE DE PORTRAITS ROYAUX EN MAINTENANCE."),
    },
    Entry {
        key: 5,
        label: "METEO DU ROYAUME",
        target: Target::Notice("PREVISION ROYALE : CHAUD, PUIS ENCORE CHAUD."),
    },
    Entry {
        key: 6,
        label: "SUITE        AUTRES SERVICES",
        target: Target::Page(PageId::Services),
    },
];

const SERVICES: &[Entry] = &[
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
        target: Target::Notice("DEMO 16 COULEURS. DEUX SONT EN GREVE."),
    },
    Entry {
        key: 5,
        label: "TELECHARGEMENTS",
        target: Target::Notice("TEMPS RESTANT: ENTRE 4 MINUTES ET JEUDI."),
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
        target: Target::Detail(DetailId::Rencontres),
    },
    Entry {
        key: 4,
        label: "PETITES ANNONCES",
        target: Target::Notice("VENDS MINITEL TBE. 18400 H. PRESQUE NEUF."),
    },
    Entry {
        key: 5,
        label: "CARNET D'ADRESSES",
        target: Target::Notice("1 CONTACT. IL VOUS A BLOQUE EN 1987."),
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
        target: Target::Notice("PANNE: ETEIGNEZ, RALLUMEZ, NIEZ L'INCIDENT."),
    },
    Entry {
        key: 5,
        label: "COMMUNIQUES",
        target: Target::Notice("COMMUNIQUE PREVU DES QU'IL N'Y AURA RIEN."),
    },
    Entry {
        key: 6,
        label: "RECTIFICATIFS",
        target: Target::Notice("RECTIFICATIF RECTIFIE. CELUI-CI AUSSI."),
    },
];

const GPE: &[Entry] = &[
    Entry {
        key: 1,
        label: "MOTEUR PIXEL",
        target: Target::Notice("GPE: PIXELS LOCAUX. AUCUN PIXEL EXTERNALISE."),
    },
    Entry {
        key: 2,
        label: "LABORATOIRE",
        target: Target::Notice("NE TOUCHEZ PAS AU FRAMEBUFFER. IL SE SOUVIENT."),
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
        target: Target::Notice("CARTE REFUSEE: ENDROIT NON GEOGRAPHIQUE."),
    },
    Entry {
        key: 4,
        label: "EXPERIENCES",
        target: Target::Notice("SUJET CONNECTE. SUJET NIE ETRE CONNECTE."),
    },
    Entry {
        key: 5,
        label: "CONTACT",
        target: Target::Notice("LE CONTACT A DEJA APPELE. VOUS AVEZ OUBLIE."),
    },
    Entry {
        key: 6,
        label: "REVENIR",
        target: Target::Summary,
    },
];

const AIDE: &[Entry] = &[
    Entry {
        key: 1,
        label: "TEST DU TERMINAL",
        target: Target::Notice("CLAVIER RECU. TERMINAL CONSIDERE COMME DOCILE."),
    },
    Entry {
        key: 2,
        label: "REVOIR CONNEXION",
        target: Target::Reconnect,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    current: PageId,
    detail: Option<DetailId>,
    pending: Option<u8>,
    history: Vec<PageId>,
    notice: Option<&'static str>,
    exit_requested: bool,
    reconnect_requested: bool,
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
            reconnect_requested: false,
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
            PageId::Services,
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
            PageId::Services => SERVICES,
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

    pub fn take_reconnect_requested(&mut self) -> bool {
        std::mem::take(&mut self.reconnect_requested)
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
            Target::Reconnect => {
                self.go_summary();
                self.reconnect_requested = true;
            }
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
