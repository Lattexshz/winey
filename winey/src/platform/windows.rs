pub enum WindowCorner {
    DoNotRound,
    SmallRound,
    Round
}

pub trait WindowExtForWindows {
    fn set_window_corner_radius(&self,corner: WindowCorner);
}