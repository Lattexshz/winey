pub enum WindowCorner {
    DoNotRound,
    SmallRound,
    Round
}

pub trait WindowExtForWindows {
    fn set_window_corner_radius(&self,corner: WindowCorner);
    fn set_window_border_color(&self,r: u8,g: u8,b: u8);
    fn set_window_caption_color(&self, r: u8, g: u8, b: u8);
    fn set_window_text_color(&self,r: u8,g: u8,b: u8);
}