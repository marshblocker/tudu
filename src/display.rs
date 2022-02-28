use crate::app::App;

pub struct Display<'a> {
    app: &'a App
}

impl<'a> Display<'a> {
    pub fn new(app: &'a App) -> Display<'a> {
        Display { app }
    }
}