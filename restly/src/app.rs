use crate::router::RouterTrait;

pub struct App;


impl App {
    pub fn new(port: &str) -> App{
        todo!()
    }

    pub fn listen<F>(port: &str, on_mounted: F) where F: FnOnce() {
        todo!()
    }
}

impl RouterTrait for App {
    fn get<F>(path: &str, handler: F) where F: Fn() {
        todo!()
    }
    fn post<F>(path: &str, handler: F) where F: Fn() {
        todo!()
    }
    fn put<F>(path: &str, handler: F) where F: Fn() {
        todo!()
    }
    fn delete<F>(path: &str, handler: F) where F: Fn() {
        todo!()
    }
    fn patch<F>(path: &str, handler: F) where F: Fn() {
        todo!()
    }
}