#![allow(unused)]

use sdk::core::structs::FLinearColor;

/// Text colors
pub const TEXT: FLinearColor = FLinearColor { r: 1f32, g: 1f32, b: 1f32, a: 1f32 };
pub const TEXT_SHADOW: FLinearColor = FLinearColor { r: 0f32, g: 0f32, b: 0f32, a: 0f32 };
pub const TEXT_OUTLINE: FLinearColor = FLinearColor { r: 0f32, g: 0f32, b: 0f32, a: 0.30 };

/// Window colors
pub const WINDOW_BACKGROUND: FLinearColor = FLinearColor { r: 0.009, g: 0.009, b: 0.009, a: 1.0 };
pub const WINDOW_HEADER: FLinearColor = FLinearColor { r: 0.05, g: 0.45, b: 0.50, a: 1.0 };

const BUTTON_IDLE: FLinearColor = FLinearColor { r: 0.10, g: 0.15, b: 0.84, a: 1.0 };
const BUTTON_HOVERED: FLinearColor = FLinearColor { r: 0.15, g: 0.20, b: 0.89, a: 1.0 };
const BUTTON_ACTIVE: FLinearColor = FLinearColor { r: 0.20, g: 0.25, b: 0.94, a: 1.0 };

const CHECKBOX_IDLE: FLinearColor = FLinearColor { r: 0.17, g: 0.16, b: 0.23, a: 1.0 };
const CHECKBOX_HOVERED: FLinearColor = FLinearColor { r: 0.22, g: 0.30, b: 0.72, a: 1.0 };
const CHECKBOX_ENABLED: FLinearColor = FLinearColor { r: 0.20, g: 0.25, b: 0.94, a: 1.0 };

const COMBOBOX_IDLE: FLinearColor = FLinearColor { r: 0.17, g: 0.16, b: 0.23, a: 1.0 };
const COMBOBOX_HOVERED: FLinearColor = FLinearColor { r: 0.17, g: 0.16, b: 0.23, a: 1.0 };
const COMBOBOX_ELEMENTS: FLinearColor = FLinearColor { r: 0.239, g: 0.42, b: 0.82, a: 1.0 };

const SLIDER_IDLE: FLinearColor = FLinearColor { r: 0.17, g: 0.16, b: 0.23, a: 1.0 };
const SLIDER_HOVERED: FLinearColor = FLinearColor { r: 0.17, g: 0.16, b: 0.23, a: 1.0 };
const SLIDER_PROGRESS: FLinearColor = FLinearColor { r: 0.22, g: 0.30, b: 0.72, a: 1.0 };
const SLIDER_BUTTON: FLinearColor = FLinearColor { r: 0.10, g: 0.15, b: 0.84, a: 1.0 };

const COLOR_PICKER_BACKGROUND: FLinearColor = FLinearColor { r: 0.006, g: 0.006, b: 0.006, a: 1.0 };
