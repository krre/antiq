use std::{cell::RefCell, rc::Rc};

use antiq::{
    Application,
    ui::{
        Color, Ui3d,
        d2::{
            geometry::Size2D,
            layout::Position2D,
            widget::{Rectangle, Widget2D},
        },
        layout::Layout,
    },
};

#[derive(Default)]
pub struct RectangleApp {}

impl Application for RectangleApp {
    fn build_ui(&self) -> Ui3d {
        let mut rect = Rectangle::new();
        rect.set_size(Size2D::new(300, 300));
        rect.set_color(Color::RED);

        let mut layout = Position2D::new();
        layout.add_widget(Rc::new(RefCell::new(rect)));

        let mut ui = Ui3d::new();
        ui.set_title("Rectangle Example");
        ui.set_layout(Box::new(layout));
        ui
    }
}

antiq::run_example_app!(RectangleApp);
