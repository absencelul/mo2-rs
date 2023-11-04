use sdk::core::structs::{FLinearColor, FVector2D};
use sdk::engine::classes::{UCanvas, UFont};

use crate::{colors, Component};

#[derive(Debug, Clone, Copy, Default)]
pub enum Alignment {
    #[default]
    Left,
    Center,
}

pub struct Label {
    font: *const UFont,
    position: FVector2D,
    alignment: Alignment,
    outline: bool,
    color: FLinearColor,
    text: String,
}

impl Label {
    pub fn new(
        position: &FVector2D,
        alignment: Alignment,
        outline: bool,
        color: &FLinearColor,
        text: &str,
    ) -> Self {
        let font = UFont::get_font();
        if font.is_null() {
            panic!("Label::new: font is null");
        }
        Self {
            font,
            position: *position,
            alignment,
            outline,
            color: *color,
            text: text.to_string(),
        }
    }

    fn draw_text_left(&self, canvas: &UCanvas) {
        canvas.k2_draw_text(
            self.font,
            self.text.as_str(),
            &self.position,
            &FVector2D { x: 0.97, y: 0.97 },
            &self.color,
            0.0,
            &colors::TEXT_SHADOW,
            &(self.position + 1f32),
            false,
            true,
            self.outline,
            &colors::TEXT_OUTLINE,
        );
    }

    fn draw_text_center(&self, canvas: &UCanvas) {
        canvas.k2_draw_text(
            self.font,
            self.text.as_str(),
            &self.position,
            &FVector2D { x: 0.97, y: 0.97 },
            &self.color,
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

impl Component for Label {
    fn draw(&self, canvas: &UCanvas) {
        match self.alignment {
            Alignment::Left => self.draw_text_left(canvas),
            Alignment::Center => self.draw_text_center(canvas),
        }
    }
}
