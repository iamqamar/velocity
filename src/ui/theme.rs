use iced::theme::Palette;
use iced::Theme;
use iced::Color;

pub fn velocity_dark() -> Theme {
    let palette = Palette {
        background: Color::from_rgb(0.08, 0.08, 0.10),
        text: Color::from_rgb(0.92, 0.92, 0.95),
        primary: Color::from_rgb(0.35, 0.55, 0.95),
        success: Color::from_rgb(0.30, 0.78, 0.48),
        danger: Color::from_rgb(0.90, 0.30, 0.35),
    };

    Theme::custom("Velocity Dark".into(), palette)
}
