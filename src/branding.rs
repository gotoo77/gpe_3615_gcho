use gotoo_pixel_engine::{Framebuffer, Image, ImageError, ImageFilter, ImageFit, Pixel, Rect};

use crate::service::Service;

const BG: Pixel = Pixel::rgb(2, 7, 10);
const DIM: Pixel = Pixel::rgb(20, 48, 55);
const OFF_WHITE: Pixel = Pixel::rgb(225, 232, 218);
const CYAN: Pixel = Pixel::rgb(50, 220, 225);

pub const ACCUEIL_LOGO_RECT: Rect = Rect {
    x: 85,
    y: 28,
    width: 150,
    height: 50,
};

pub fn decode_minitel_logo() -> Result<Image, ImageError> {
    Image::decode_png(include_bytes!(
        "../assets/branding/3615_gcho_logo_minitel.png"
    ))
}

pub(crate) fn render_accueil_branding(
    framebuffer: &mut Framebuffer,
    service: &Service,
    logo: &Image,
) {
    // Replace only the accueil content band. The terminal frame, header,
    // public-service banner, live status and function keys remain owned by
    // their existing renderers.
    framebuffer.fill_rect(9, 22, 302, 138, BG);
    for y in (22..160).step_by(4) {
        framebuffer.draw_line(9, y, 310, y, Pixel::rgb(3, 12, 15));
    }

    framebuffer.draw_line(66, 25, 254, 25, DIM);
    framebuffer.draw_image_fit(
        logo,
        ACCUEIL_LOGO_RECT,
        ImageFit::Contain,
        ImageFilter::Nearest,
    );
    framebuffer.draw_line(66, 79, 254, 79, DIM);
    draw_center(
        framebuffer,
        81,
        "SERVICE VIDEOTEX GCHO // CANAL OUVERT",
        DIM,
    );

    for (index, entry) in service.entries().iter().enumerate() {
        let y = 91 + index as i32 * 10;
        let selected = service.pending_digit() == Some(entry.key);
        if selected {
            framebuffer.fill_rect(9, y - 2, 302, 10, CYAN);
        }
        let ink = if selected { BG } else { OFF_WHITE };
        framebuffer.draw_text(
            14,
            y,
            &format!("{}  {}", entry.key, fit(entry.label, 43)),
            ink,
        );
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
