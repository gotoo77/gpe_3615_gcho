use gotoo_pixel_engine::{Framebuffer, Image, ImageError, ImageFilter, ImageFit, Pixel, Rect};

use crate::service::Service;

const BG: Pixel = Pixel::rgb(2, 7, 10);
const DIM: Pixel = Pixel::rgb(20, 48, 55);
const OFF_WHITE: Pixel = Pixel::rgb(225, 232, 218);
const CYAN: Pixel = Pixel::rgb(50, 220, 225);
const BLUE: Pixel = Pixel::rgb(80, 130, 255);
const RED: Pixel = Pixel::rgb(245, 85, 75);
const YELLOW: Pixel = Pixel::rgb(245, 215, 70);

pub const ACCUEIL_LOGO_RECT: Rect = Rect {
    x: 85,
    y: 28,
    width: 150,
    height: 50,
};

pub const ACCUEIL_MENU_LEFT_X: i32 = 98;
pub const ACCUEIL_MENU_RIGHT_X: i32 = 311;
pub const ACCUEIL_MENU_START_Y: i32 = 94;
pub const ACCUEIL_MENU_STEP: i32 = 16;
pub const ACCUEIL_MENU_ROW_HEIGHT: i32 = 12;

const SPLASH_LOGO_RECT: Rect = Rect {
    x: 40,
    y: 72,
    width: 240,
    height: 80,
};

pub fn decode_minitel_logo() -> Result<Image, ImageError> {
    Image::decode_png(include_bytes!(
        "../assets/branding/3615_gcho_logo_minitel.png"
    ))
}

pub fn decode_macronus_portrait() -> Result<Image, ImageError> {
    Image::decode_png(include_bytes!("../assets/portraits/macronus_ier.png"))
}

pub(crate) fn render_branding_splash(framebuffer: &mut Framebuffer, logo: &Image) {
    framebuffer.clear(BG);
    for y in (0..framebuffer.height() as i32).step_by(4) {
        framebuffer.draw_line(
            0,
            y,
            framebuffer.width() as i32 - 1,
            y,
            Pixel::rgb(3, 12, 15),
        );
    }

    draw_center(framebuffer, 34, "CONNEXION ETABLIE", CYAN);
    framebuffer.draw_line(40, 59, 279, 59, DIM);
    framebuffer.draw_image_fit(
        logo,
        SPLASH_LOGO_RECT,
        ImageFit::Contain,
        ImageFilter::Nearest,
    );
    framebuffer.draw_line(40, 164, 279, 164, DIM);
    draw_center(framebuffer, 177, "SERVICE VIDEOTEX GCHO", OFF_WHITE);
    draw_center(framebuffer, 204, "ENTREE / ECHAP : PASSER", DIM);
}

pub(crate) fn render_accueil_branding(
    framebuffer: &mut Framebuffer,
    service: &Service,
    logo: &Image,
    macronus_portrait: &Image,
) {
    framebuffer.fill_rect(9, 22, 302, 176, BG);
    for y in (22..198).step_by(4) {
        framebuffer.draw_line(9, y, 310, y, Pixel::rgb(3, 12, 15));
    }

    framebuffer.draw_line(14, 25, 109, 25, BLUE);
    framebuffer.draw_line(110, 25, 209, 25, OFF_WHITE);
    framebuffer.draw_line(210, 25, 305, 25, RED);
    framebuffer.draw_image_fit(
        logo,
        ACCUEIL_LOGO_RECT,
        ImageFit::Contain,
        ImageFilter::Nearest,
    );
    draw_center(framebuffer, 78, "SERVICE TELEMATIQUE CLIMATIQUE", OFF_WHITE);

    render_royal_portrait(framebuffer, macronus_portrait);

    for (index, entry) in service.entries().iter().enumerate() {
        let y = ACCUEIL_MENU_START_Y + index as i32 * ACCUEIL_MENU_STEP;
        if service.pending_digit() == Some(entry.key) {
            framebuffer.fill_rect(
                ACCUEIL_MENU_LEFT_X,
                y - 2,
                (ACCUEIL_MENU_RIGHT_X - ACCUEIL_MENU_LEFT_X) as u32,
                ACCUEIL_MENU_ROW_HEIGHT as u32,
                BLUE,
            );
        }
        framebuffer.draw_text(
            ACCUEIL_MENU_LEFT_X + 5,
            y,
            &format!("{} {}", entry.key, fit(entry.label, 31)),
            OFF_WHITE,
        );
    }

    framebuffer.draw_line(98, 190, 305, 190, DIM);
    framebuffer.draw_text(101, 192, "FICTION TELEMATIQUE // CANAL ROYAL", DIM);

    if let Some(notice) = service.notice() {
        render_home_notice(framebuffer, notice);
    }
}

fn render_royal_portrait(framebuffer: &mut Framebuffer, portrait: &Image) {
    const LEFT: i32 = 14;
    const TOP: i32 = 94;

    framebuffer.draw_rect(LEFT, TOP, 76, 94, DIM);
    framebuffer.fill_rect(LEFT + 3, TOP + 3, 23, 4, BLUE);
    framebuffer.fill_rect(LEFT + 26, TOP + 3, 23, 4, OFF_WHITE);
    framebuffer.fill_rect(LEFT + 49, TOP + 3, 24, 4, RED);
    framebuffer.draw_image_fit(
        portrait,
        Rect {
            x: LEFT + 3,
            y: TOP + 9,
            width: 70,
            height: 82,
        },
        ImageFit::Contain,
        ImageFilter::Nearest,
    );
}

fn render_home_notice(framebuffer: &mut Framebuffer, notice: &str) {
    framebuffer.fill_rect(98, 168, 213, 30, Pixel::rgb(8, 24, 28));
    framebuffer.draw_rect(98, 168, 213, 30, RED);
    framebuffer.draw_text(103, 174, &fit(notice, 31), YELLOW);
    if notice.chars().count() > 31 {
        let rest: String = notice.chars().skip(31).collect();
        framebuffer.draw_text(103, 185, &fit(rest.trim_start(), 31), YELLOW);
    }
}

fn draw_center(framebuffer: &mut Framebuffer, y: i32, text: &str, color: Pixel) {
    let (width, _) = Framebuffer::text_size(text, 1);
    let x = ((framebuffer.width().saturating_sub(width)) / 2) as i32;
    framebuffer.draw_text(x, y, text, color);
}

fn fit(text: impl AsRef<str>, max_chars: usize) -> String {
    text.as_ref().chars().take(max_chars).collect()
}
