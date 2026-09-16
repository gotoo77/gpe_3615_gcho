use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EditorialMessage {
    pub id: String,
    pub category: String,
    pub title: String,
    pub body: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ContentFile {
    pub generated_at: String,
    pub messages: Vec<EditorialMessage>,
}

impl ContentFile {
    pub fn from_json(source: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(source)
    }

    fn is_usable(&self) -> bool {
        !self.generated_at.trim().is_empty()
            && !self.messages.is_empty()
            && self.messages.iter().all(|message| {
                !message.id.trim().is_empty()
                    && !message.category.trim().is_empty()
                    && !message.title.trim().is_empty()
                    && !message.body.is_empty()
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentBundle {
    pub service_public: ContentFile,
    pub news: ContentFile,
    pub messages: ContentFile,
    pub secrets: ContentFile,
    pub rencontres: ContentFile,
}

impl ContentBundle {
    pub fn load_bundled() -> Self {
        Self::from_sources_with_rencontres(
            include_str!("../content/service_public.json"),
            include_str!("../content/news.json"),
            include_str!("../content/messages.json"),
            include_str!("../content/secrets.json"),
            include_str!("../content/rencontres.json"),
        )
    }

    pub fn from_sources(service_public: &str, news: &str, messages: &str, secrets: &str) -> Self {
        Self::from_sources_with_rencontres(service_public, news, messages, secrets, "")
    }

    fn from_sources_with_rencontres(
        service_public: &str,
        news: &str,
        messages: &str,
        secrets: &str,
        rencontres: &str,
    ) -> Self {
        Self {
            service_public: parse_or(service_public, fallback_service_public),
            news: parse_or(news, fallback_news),
            messages: parse_or(messages, fallback_messages),
            secrets: parse_or(secrets, fallback_secrets),
            rencontres: parse_or(rencontres, fallback_rencontres),
        }
    }
}

fn parse_or(source: &str, fallback: fn() -> ContentFile) -> ContentFile {
    ContentFile::from_json(source)
        .ok()
        .filter(ContentFile::is_usable)
        .unwrap_or_else(fallback)
}

fn message(id: &str, category: &str, title: &str, body: &[&str]) -> EditorialMessage {
    EditorialMessage {
        id: id.into(),
        category: category.into(),
        title: title.into(),
        body: body.iter().map(|line| (*line).into()).collect(),
    }
}

fn fallback_file(message: EditorialMessage) -> ContentFile {
    ContentFile {
        generated_at: "bundled-fallback".into(),
        messages: vec![message],
    }
}

fn fallback_service_public() -> ContentFile {
    fallback_file(message(
        "fallback-service-public",
        "service_public",
        "MESSAGE DU SERVICE PUBLIC",
        &["VOUS AVEZ CHAUD ?", "PENSEZ A BOIRE DE L'EAU."],
    ))
}

fn fallback_news() -> ContentFile {
    fallback_file(message(
        "fallback-news",
        "breve",
        "DERNIERE MINUTE",
        &["LE RESEAU EST TOUJOURS LA."],
    ))
}

fn fallback_messages() -> ContentFile {
    fallback_file(message(
        "fallback-message",
        "messagerie",
        "BOITE DE RECEPTION",
        &["1 MESSAGE NON LU", "EXPEDITEUR: INCONNU"],
    ))
}

fn fallback_secrets() -> ContentFile {
    fallback_file(message(
        "fallback-secret",
        "noeud7",
        "MESSAGE DU MINISTERE DE LA NORMALITE",
        &["LE TERMINAL NE BOURDONNE PAS."],
    ))
}

fn fallback_rencontres() -> ContentFile {
    fallback_file(message(
        "fallback-rencontres",
        "rencontres",
        "SERVICE RENCONTRES",
        &[
            "AUCUN PROFIL DISPONIBLE.",
            "LA FACTURATION, ELLE, FONCTIONNE.",
        ],
    ))
}
