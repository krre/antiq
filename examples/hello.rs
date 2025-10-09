use antiq::{Application, ui::Ui3d};

#[derive(Default)]
pub struct HelloApp {}

impl Application for HelloApp {
    fn build_ui(&self) -> Ui3d {
        let mut ui = Ui3d::new();
        ui.set_title("Hello Example");
        ui
    }
}

antiq::run_example_app!(HelloApp);
