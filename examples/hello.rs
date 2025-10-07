use antiq::{Application, ui::Ui3d};

#[derive(Default)]
pub struct HelloApp {}

impl Application for HelloApp {
    fn build_ui(&self) -> Ui3d {
        Ui3d::new()
    }
}

antiq::run_example_app!(HelloApp);
