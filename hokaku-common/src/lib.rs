pub enum PixelFormat {
    RGBA
}

pub trait IsPixelFormat {
    fn get_bpp(self) -> u8;
}

impl IsPixelFormat for PixelFormat {
    fn get_bpp(self) -> u8 {
        match self {
            PixelFormat::RGBA => {
                return 32;
            }
        }
    }
}