use sdk::core::structs::{FLinearColor, FVector2D};
use sdk::engine::classes::{UCanvas, UFont};

use crate::components::rect::{Rect, RectType};
use crate::{colors, Component};

pub struct Window {
    title: String,
    position: FVector2D,
    size: FVector2D,
    color: FLinearColor,
    opened: bool,
}

impl Window {
    pub fn new(
        title: &str,
        position: &FVector2D,
        size: &FVector2D,
        color: &FLinearColor,
        opened: bool,
    ) -> Self {
        Self { title: title.to_string(), position: *position, size: *size, color: *color, opened }
    }

    pub fn toggle_window(&mut self) {
        self.opened = !self.opened;
    }
}

impl Component for Window {
    fn draw(&self, canvas: &UCanvas) {
        if !self.opened {
            return;
        }

        // draw background
        let bg =
            Rect::new(&self.position, &self.size, &colors::WINDOW_BACKGROUND, RectType::Filled);
        bg.draw_rect_filled(canvas);

        // draw header
        let header = Rect::new(
            &self.position,
            &FVector2D { x: self.size.x, y: 30f32 },
            &colors::WINDOW_HEADER,
            RectType::Filled,
        );
        header.draw_rect_filled(canvas);

        // draw title
        let title_position = FVector2D::new(
            self.position.x + self.size.x / 2.0f32,
            self.position.y + 25.0f32 / 2.0f32,
        );
        let font = UFont::get_font();
        if !font.is_null() {
            canvas.k2_draw_text(
                font,
                self.title.as_str(),
                &title_position,
                &FVector2D { x: 0.97, y: 0.97 },
                &colors::TEXT,
                1.0f32,
                &colors::TEXT_SHADOW,
                &(self.position + 1.0),
                true,
                true,
                true,
                &colors::TEXT_OUTLINE,
            );
        }
    }
}
