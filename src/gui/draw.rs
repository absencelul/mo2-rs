// use crate::sdk::core::{FLinearColor, FVector2D};
// use crate::sdk::engine::UCanvas;
//
// #[derive(Debug, Clone, Copy)]
// pub enum DrawItemType {
//     None,
//     FilledRect,
//     TextLeft,
//     TextCenter,
//     TextRight,
//     Line,
// }
//
// #[derive(Debug, Clone)]
// pub struct DrawItem {
//     pub kind: DrawItemType,
//     pub position: FVector2D,
//     pub size: FVector2D,
//     pub color: FLinearColor,
// }
//
// impl DrawItem {
//     fn new_empty() -> Self {
//         Self {
//             kind: DrawItemType::None,
//             position: FVector2D::new(0f32, 0f32),
//             size: FVector2D::new(0f32, 0f32),
//             color: FLinearColor::new(0f32, 0f32, 0f32, 0f32),
//         }
//     }
// }
//
// pub struct Gui {
//     draw_list: [DrawItem; 128],
//     canvas: *const UCanvas,
// }
//
// impl Gui {
//     pub fn new(canvas: &UCanvas) -> Self {
//         Self {
//             draw_list: [DrawItem::new_empty(); 128],
//             canvas,
//         }
//     }
//
//     // pub fn text_left()
//
//     pub fn draw_line(&self, from: FVector2D, to: FVector2D, color: FLinearColor, thickness: f32) {
//         self.canvas.draw_line(from, to, color, thickness);
//     }
// }
