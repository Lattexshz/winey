pub mod window;
pub mod platform;

pub trait WineyWindowImplementation {
    fn show(&self);
    fn hide(&self);
    fn set_maximize(&self,maximize: bool);
    fn set_minimize(&self,minimize: bool);
    fn set_title(&self,title: &str);
    fn set_undecorated(&self,undecorated: bool);
    fn run<C:FnMut()>(&self,callback: C);
}