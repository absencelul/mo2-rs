use sdk::engine::classes::UCanvas;

pub mod colors;
pub mod components;

pub struct Gui {
    canvas: *const UCanvas,
    components: Vec<Box<dyn Component>>,
}

impl Gui {
    pub fn new(canvas: &UCanvas) -> Self {
        Self { canvas, components: Vec::new() }
    }

    pub fn render(&self) {
        self.components.iter().for_each(|component| {
            component.draw(unsafe { &*self.canvas });
        });
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }
}

pub trait Component {
    fn draw(&self, canvas: &UCanvas);
}
