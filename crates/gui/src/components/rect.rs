use sdk::core::structs::{FLinearColor, FVector2D};
use sdk::engine::classes::UCanvas;

use crate::Component;

pub enum RectType {
    Filled,
    Outlined,
}

pub struct Rect {
    position: FVector2D,
    size: FVector2D,
    color: FLinearColor,
    rect_type: RectType,
}

impl Rect {
    pub fn new(
        position: &FVector2D,
        size: &FVector2D,
        color: &FLinearColor,
        rect_type: RectType,
    ) -> Self {
        Self { position: *position, size: *size, color: *color, rect_type }
    }

    fn draw_rect_outline(&self, canvas: &UCanvas) {
        canvas.k2_draw_box(&self.position, &self.size, 1.0f32, &self.color);
    }

    pub fn draw_rect_filled(&self, canvas: &UCanvas) {
        for i in 0..self.size.y as i32 {
            canvas.k2_draw_line(
                &FVector2D { x: self.position.x, y: self.position.y + i as f32 },
                &FVector2D { x: self.position.x + self.size.x, y: self.position.y + i as f32 },
                1.0f32,
                &self.color,
            );
        }
    }
}

impl Component for Rect {
    fn draw(&self, canvas: &UCanvas) {
        match self.rect_type {
            RectType::Filled => self.draw_rect_filled(canvas),
            RectType::Outlined => self.draw_rect_outline(canvas),
        }
    }
}
