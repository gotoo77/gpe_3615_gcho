use gotoo_pixel_engine::{Framebuffer, Pixel};

use crate::content::{ContentBundle, ContentFile};
use crate::facade::{FunctionKey, function_keys, snapshot_date_label};
use crate::service::{DetailId, PageId, Service};

const BG: Pixel = Pixel::rgb(2, 7, 10);
const DIM: Pixel = Pixel::rgb(20, 48, 55);
const OFF_WHITE: Pixel = Pixel::rgb(225, 232, 218);
const CYAN: Pixel = Pixel::rgb(50, 220, 225);
const BLUE: Pixel = Pixel::rgb(80, 130, 255);
const GREEN: Pixel = Pixel::rgb(80, 230, 120);
const RED: Pixel = Pixel::rgb(245, 85, 75);
const YELLOW: Pixel = Pixel::rgb(245, 215, 70);

pub fn render_boot(framebuffer: &mut Framebuffer, progress: f32) {
    terminal_background(framebuffer);
    frame(framebuffer);

    draw_center(framebuffer, 28, "CONNEXION AU SERVICE", CYAN, 1);
    draw_center(framebuffer, 62, "3615", YELLOW, 2);
    draw_center(framebuffer, 84, "GCHO", CYAN, 2);
    draw_center(
        framebuffer,
        122,
        "INITIALISATION DU TERMINAL...",
        OFF_WHITE,
        1,
    );

    let width = 180_u32;
    let filled = (width as f32 * progress.clamp(0.0, 1.0)).round() as u32;
    framebuffer.draw_rect(70, 143, width, 10, BLUE);
    if filled > 2 {
        framebuffer.fill_rect(72, 145, filled.saturating_sub(2).min(width - 4), 6, GREEN);
    }

    let status = if progress < 0.30 {
        "NUMEROTATION : 3615"
    } else if progress < 0.72 {
        "NEGOCIATION DU PROTOCOLE..."
    } else {
        "CONNEXION ETABLIE"
    };
    draw_center(framebuffer, 171, status, OFF_WHITE, 1);
    draw_center(framebuffer, 202, "VEUILLEZ PATIENTER", DIM, 1);
}

pub fn render_terminal(
    framebuffer: &mut Framebuffer,
    service: &Service,
    content: &ContentBundle,
    cursor_visible: bool,
    active_function_key: Option<FunctionKey>,
) {
    terminal_background(framebuffer);
    frame(framebuffer);

    let page = service.current_page();
    let (page_number, title) = page_identity(page);
    framebuffer.draw_text(10, 9, "3615 GCHO", CYAN);
    framebuffer.draw_text(258, 9, page_number, DIM);
    framebuffer.draw_line(8, 20, 311, 20, BLUE);

    if let Some(detail) = service.current_detail() {
        draw_center(framebuffer, 29, detail_title(detail), YELLOW, 1);
        render_detail(framebuffer, detail, content);
        render_notice(framebuffer, service.notice());
        render_function_keys(framebuffer, active_function_key);
        return;
    }

    draw_center(framebuffer, 29, title, YELLOW, 1);
    if page == PageId::Accueil {
        draw_center(framebuffer, 41, "L'ESPRIT EST EN RESEAU", GREEN, 1);
    }

    let start_y = if page == PageId::Accueil { 61 } else { 52 };
    for (index, entry) in service.entries().iter().enumerate() {
        let y = start_y + index as i32 * 15;
        let selected = service.pending_digit() == Some(entry.key);
        if selected {
            framebuffer.fill_rect(9, y - 3, 302, 12, CYAN);
        }
        let ink = if selected { BG } else { OFF_WHITE };
        framebuffer.draw_text(
            14,
            y,
            &format!("{}  {}", entry.key, fit(entry.label, 43)),
            ink,
        );
    }

    render_page_content(framebuffer, page, content);
    render_notice(framebuffer, service.notice());

    if service.notice().is_none() {
        framebuffer.draw_text(11, 198, "VOTRE CHOIX ?", OFF_WHITE);
        if let Some(digit) = service.pending_digit() {
            framebuffer.draw_text(96, 198, &digit.to_string(), CYAN);
        }
        if cursor_visible {
            framebuffer.fill_rect(104, 197, 5, 8, CYAN);
        }
    }
    render_function_keys(framebuffer, active_function_key);
}

fn render_detail(framebuffer: &mut Framebuffer, detail: DetailId, content: &ContentBundle) {
    match detail {
        DetailId::ArcadeScores => {
            section_rule(framebuffer, 48, "TABLE DES SCORES", GREEN);
            text(framebuffer, 62, "01  ADMIN ............... 999999", YELLOW);
            text(
                framebuffer,
                76,
                "02  XAVIER_92 ........... 042361",
                OFF_WHITE,
            );
            text(
                framebuffer,
                90,
                "03  GUEST ............... 000120",
                OFF_WHITE,
            );
            text(framebuffer, 116, "CREDITS DISPONIBLES : 00", CYAN);
            text(framebuffer, 132, "POUR JOUER, VEUILLEZ INSERER", OFF_WHITE);
            text(framebuffer, 144, "UNE PIECE DANS VOTRE MINITEL.", OFF_WHITE);
        }
        DetailId::PixelMaze => {
            section_rule(framebuffer, 48, "PIXEL MAZE V1.2", GREEN);
            text(framebuffer, 64, "+-------+-------+-------+", CYAN);
            text(framebuffer, 78, "|   @   |       | SORTIE |", OFF_WHITE);
            text(framebuffer, 92, "| +---+ | +---+ | +---+ |", OFF_WHITE);
            text(framebuffer, 106, "|     |   |       |     |", OFF_WHITE);
            text(framebuffer, 120, "+---+ +---+ +---+ +---+ +", OFF_WHITE);
            text(framebuffer, 146, "PARTIE SUSPENDUE DEPUIS 1987.", YELLOW);
            text(framebuffer, 162, "TEMPS ECOULE : 341773 HEURES", DIM);
        }
        DetailId::Mailbox => {
            section_rule(framebuffer, 48, "BOITE DE RECEPTION", GREEN);
            render_editorial_full(framebuffer, 62, &content.messages, GREEN);
            text(framebuffer, 150, "ETAT : NON LU", YELLOW);
            text(framebuffer, 164, "REPONSE IMPOSSIBLE :", OFF_WHITE);
            text(
                framebuffer,
                176,
                "DESTINATAIRE HORS LIGNE DEPUIS 1992.",
                OFF_WHITE,
            );
        }
        DetailId::ChatRooms => {
            section_rule(framebuffer, 48, "SALONS DISPONIBLES", GREEN);
            text(framebuffer, 64, "#GENERAL ........ 03 CONNECTES", CYAN);
            text(framebuffer, 80, "#JEUX ........... 01 CONNECTE", OFF_WHITE);
            text(framebuffer, 96, "#MINITEL ........ 11 FANTOMES", OFF_WHITE);
            text(framebuffer, 112, "#SECRET ......... ACCES REFUSE", RED);
            text(framebuffer, 142, "DERNIER MESSAGE : 17:42  12/05/1996", DIM);
            text(framebuffer, 158, "PERSONNE N'A QUITTE LE SALON.", YELLOW);
        }
        DetailId::News => {
            section_rule(framebuffer, 48, "FIL DES BREVES", CYAN);
            render_editorial_full(framebuffer, 62, &content.news, CYAN);
            text(framebuffer, 158, "SOURCE : SNAPSHOT EDITORIAL", DIM);
            text(
                framebuffer,
                172,
                &snapshot_date_label(&content.news.generated_at),
                CYAN,
            );
            text(framebuffer, 186, "MAJ AU PROCHAIN RAFRAICHISSEMENT.", DIM);
        }
        DetailId::ServicePublic => {
            section_rule(framebuffer, 48, "TRANSMISSION OFFICIELLE", CYAN);
            render_editorial_full(framebuffer, 62, &content.service_public, CYAN);
            text(framebuffer, 162, "MESSAGE AUTOMATIQUEMENT APPROUVE", DIM);
            text(framebuffer, 174, "PAR LE MINISTERE DE LA NORMALITE.", DIM);
        }
        DetailId::Alerts => {
            section_rule(framebuffer, 48, "CENTRE NATIONAL D'ALERTE", CYAN);
            text(
                framebuffer,
                66,
                "NIVEAU : ADMINISTRATIVEMENT VIGILANT",
                YELLOW,
            );
            text(
                framebuffer,
                86,
                "SECTEUR : ENSEMBLE DU TERRITOIRE",
                OFF_WHITE,
            );
            text(
                framebuffer,
                102,
                "MOTIF : SITUATION A SURVEILLER",
                OFF_WHITE,
            );
            text(framebuffer, 126, "CONSIGNE 1 : RESTEZ INFORME", CYAN);
            text(framebuffer, 142, "CONSIGNE 2 : NE PANIQUEZ PAS", CYAN);
            text(framebuffer, 158, "CONSIGNE 3 : PANIQUE NON HOMOLOGUEE", DIM);
        }
        DetailId::GpeProjects => {
            section_rule(framebuffer, 48, "PROJETS ACCESSIBLES", GREEN);
            text(framebuffer, 62, "GPE .......... MOTEUR PIXEL", CYAN);
            text(framebuffer, 78, "DRUID ........ CANAL FANTOME", OFF_WHITE);
            text(
                framebuffer,
                94,
                "VOID ......... TRANSMISSION VERTICALE",
                OFF_WHITE,
            );
            text(
                framebuffer,
                110,
                "SIX-SEVEN .... PROTOCOLE NON DOCUMENTE",
                OFF_WHITE,
            );
            text(
                framebuffer,
                138,
                "STATUT : EN CONSTRUCTION PERMANENTE",
                GREEN,
            );
            text(framebuffer, 154, "CERTAINS SERVICES PEUVENT EXISTER", DIM);
            text(framebuffer, 166, "AVANT LEUR DATE DE CREATION.", DIM);
        }
        DetailId::Node7File => {
            section_rule(framebuffer, 48, "DOSSIER 7/7", RED);
            render_editorial_full(framebuffer, 62, &content.secrets, RED);
            text(framebuffer, 148, "ORIGINE DU SIGNAL : ICI", YELLOW);
            text(framebuffer, 164, "DATE D'OUVERTURE : 00/00/0000", OFF_WHITE);
            text(framebuffer, 180, "DATE DE FERMETURE : EN COURS", OFF_WHITE);
            text(framebuffer, 198, "NE DECONNECTEZ PAS LE TERMINAL.", RED);
        }
        DetailId::Node7Witnesses => {
            section_rule(framebuffer, 48, "DEPOSITIONS ARCHIVEES", RED);
            text(framebuffer, 64, "TEMOIN 01 : N'A RIEN VU", OFF_WHITE);
            text(
                framebuffer,
                80,
                "TEMOIN 02 : CONFIRME N'AVOIR RIEN VU",
                OFF_WHITE,
            );
            text(framebuffer, 96, "TEMOIN 03 : N'EXISTE PAS", OFF_WHITE);
            text(
                framebuffer,
                120,
                "CONCORDANCE DES TEMOIGNAGES : 100%",
                YELLOW,
            );
            text(framebuffer, 144, "INCIDENT OBSERVE : AUCUN", DIM);
            text(framebuffer, 160, "DOSSIER CLASSE : NON", RED);
            text(framebuffer, 176, "RAISON : TEMOINS TROP COHERENTS", RED);
        }
    }
}

fn render_page_content(framebuffer: &mut Framebuffer, page: PageId, content: &ContentBundle) {
    match page {
        PageId::Infos => render_editorial(framebuffer, 146, &content.news, CYAN),
        PageId::Messagerie => render_editorial(framebuffer, 143, &content.messages, GREEN),
        PageId::Noeud7 => render_editorial(framebuffer, 146, &content.secrets, RED),
        PageId::Aide => {
            framebuffer.draw_text(14, 88, "0-9       CHOIX", OFF_WHITE);
            framebuffer.draw_text(14, 101, "ENTREE     ENVOI", OFF_WHITE);
            framebuffer.draw_text(14, 114, "RETOUR ARR CORRECTION", OFF_WHITE);
            framebuffer.draw_text(14, 127, "ECHAP      RETOUR", OFF_WHITE);
            framebuffer.draw_text(14, 140, "DEBUT/HOME SOMMAIRE", OFF_WHITE);
            framebuffer.draw_text(14, 153, "H          GUIDE", OFF_WHITE);
        }
        PageId::Gpe => {
            framebuffer.draw_text(14, 114, "RUNTIME: GPE / RUST / WEBGPU", GREEN);
            framebuffer.draw_text(14, 127, "ETAT: L'ILLUSION FONCTIONNE", CYAN);
        }
        PageId::Arcade => {
            framebuffer.draw_text(14, 146, "CREDITS: 00", GREEN);
            framebuffer.draw_text(14, 159, "HAUT SCORE: ADMIN", YELLOW);
        }
        PageId::Accueil => framebuffer.draw_text(14, 172, "RESEAU GCHO: OUVERT", GREEN),
    }
}

fn render_notice(framebuffer: &mut Framebuffer, notice: Option<&str>) {
    let Some(notice) = notice else {
        return;
    };
    framebuffer.fill_rect(9, 184, 302, 26, Pixel::rgb(8, 24, 28));
    framebuffer.draw_rect(9, 184, 302, 26, RED);
    framebuffer.draw_text(14, 190, &fit(notice, 46), YELLOW);
    if notice.chars().count() > 46 {
        let rest: String = notice.chars().skip(46).collect();
        framebuffer.draw_text(14, 200, &fit(rest.trim_start(), 46), YELLOW);
    }
}

fn render_function_keys(framebuffer: &mut Framebuffer, active: Option<FunctionKey>) {
    for spec in function_keys() {
        let pressed = active == Some(spec.key);
        let (face, border, ink) = match (spec.key, pressed) {
            (FunctionKey::Send, false) => (Pixel::rgb(12, 62, 30), GREEN, GREEN),
            (FunctionKey::Correction, false) => (Pixel::rgb(48, 40, 10), YELLOW, YELLOW),
            (_, false) => (Pixel::rgb(10, 28, 34), DIM, OFF_WHITE),
            (FunctionKey::Send, true) => (GREEN, OFF_WHITE, BG),
            (FunctionKey::Correction, true) => (YELLOW, OFF_WHITE, BG),
            (_, true) => (CYAN, OFF_WHITE, BG),
        };

        framebuffer.fill_rect(
            spec.x + 2,
            spec.y + 2,
            spec.width as u32,
            spec.height as u32,
            Pixel::rgb(1, 4, 5),
        );
        framebuffer.fill_rect(
            spec.x,
            spec.y,
            spec.width as u32,
            spec.height as u32,
            face,
        );
        framebuffer.draw_rect(
            spec.x,
            spec.y,
            spec.width as u32,
            spec.height as u32,
            border,
        );
        framebuffer.draw_line(
            spec.x + 2,
            spec.y + 2,
            spec.x + spec.width - 3,
            spec.y + 2,
            if pressed { OFF_WHITE } else { DIM },
        );

        let (label_width, _) = Framebuffer::text_size(spec.label, 1);
        let label_x = spec.x + ((spec.width - label_width as i32) / 2).max(2);
        framebuffer.draw_text(label_x, spec.y + 7, spec.label, ink);
    }
}

fn render_editorial(framebuffer: &mut Framebuffer, y: i32, file: &ContentFile, accent: Pixel) {
    let Some(message) = file.messages.first() else {
        return;
    };
    framebuffer.draw_text(14, y, &fit(&message.title, 46), accent);
    for (index, line) in message.body.iter().take(2).enumerate() {
        framebuffer.draw_text(14, y + 12 + index as i32 * 10, &fit(line, 46), OFF_WHITE);
    }
}

fn render_editorial_full(framebuffer: &mut Framebuffer, y: i32, file: &ContentFile, accent: Pixel) {
    let Some(message) = file.messages.first() else {
        text(framebuffer, y, "AUCUNE DONNEE DISPONIBLE.", RED);
        return;
    };
    text(framebuffer, y, &fit(&message.title, 46), accent);
    for (index, line) in message.body.iter().take(5).enumerate() {
        text(
            framebuffer,
            y + 16 + index as i32 * 13,
            &fit(line, 46),
            OFF_WHITE,
        );
    }
}

fn section_rule(framebuffer: &mut Framebuffer, y: i32, label: &str, color: Pixel) {
    framebuffer.draw_text(14, y, label, color);
    framebuffer.draw_line(14, y + 10, 305, y + 10, DIM);
}

fn text(framebuffer: &mut Framebuffer, y: i32, value: &str, color: Pixel) {
    framebuffer.draw_text(14, y, &fit(value, 46), color);
}

fn terminal_background(framebuffer: &mut Framebuffer) {
    framebuffer.clear(BG);
    for y in (2..framebuffer.height() as i32).step_by(4) {
        framebuffer.draw_line(
            0,
            y,
            framebuffer.width() as i32 - 1,
            y,
            Pixel::rgb(3, 12, 15),
        );
    }
}

fn frame(framebuffer: &mut Framebuffer) {
    framebuffer.draw_rect(4, 4, framebuffer.width() - 8, framebuffer.height() - 8, DIM);
}

fn draw_center(framebuffer: &mut Framebuffer, y: i32, text: &str, color: Pixel, scale: u32) {
    let (width, _) = Framebuffer::text_size(text, scale);
    let x = ((framebuffer.width().saturating_sub(width)) / 2) as i32;
    framebuffer.draw_text_scaled(x, y, text, scale, color);
}

fn fit(text: impl AsRef<str>, max_chars: usize) -> String {
    text.as_ref().chars().take(max_chars).collect()
}

fn detail_title(detail: DetailId) -> &'static str {
    match detail {
        DetailId::ArcadeScores => "ARCADE / CLASSEMENT",
        DetailId::PixelMaze => "ARCADE / PIXEL MAZE",
        DetailId::Mailbox => "MESSAGERIE / MA BOITE",
        DetailId::ChatRooms => "MESSAGERIE / SALONS",
        DetailId::News => "INFOS / BREVES",
        DetailId::ServicePublic => "INFOS / SERVICE PUBLIC",
        DetailId::Alerts => "INFOS / ALERTES",
        DetailId::GpeProjects => "GPE / PROJETS",
        DetailId::Node7File => "NOEUD 7 / LE FICHIER",
        DetailId::Node7Witnesses => "NOEUD 7 / LES TEMOINS",
    }
}

fn page_identity(page: PageId) -> (&'static str, &'static str) {
    match page {
        PageId::Accueil => ("01/07", "3615 GCHO"),
        PageId::Arcade => ("02/07", "ARCADE"),
        PageId::Messagerie => ("03/07", "MESSAGERIE"),
        PageId::Infos => ("04/07", "INFOS"),
        PageId::Gpe => ("05/07", "GPE"),
        PageId::Noeud7 => ("06/07", "NOEUD 7"),
        PageId::Aide => ("07/07", "AIDE"),
    }
}
