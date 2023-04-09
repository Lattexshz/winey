pub enum WindowCorner {
    DoNotRound,
    SmallRound,
    Round
}

pub trait WindowExtForWindows {
    /// Specifies how much to round the corners of the window.
    /// This method is supported in Windows 11 build 22000 or later due to DwmAPI reasons.
    fn set_window_corner_radius(&self,corner: WindowCorner);
    /// Specifies the color of the window border using RGB values
    /// This method is supported in Windows 11 build 22000 or later due to DwmAPI reasons.
    fn set_window_border_color(&self,r: u8,g: u8,b: u8);
    /// Specifies the color of the caption using RGB values
    /// This method is supported in Windows 11 build 22000 or later due to DwmAPI reasons.
    fn set_window_caption_color(&self, r: u8, g: u8, b: u8);
    /// Specifies the color of the window text( title ) using RGB values
    /// This method is supported in Windows 11 build 22000 or later due to DwmAPI reasons.
    fn set_window_text_color(&self,r: u8,g: u8,b: u8);
}