use display_interface::DisplayError;
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, DrawTarget},
    primitives::Rectangle,
    Pixel,
};
use hal::{
    gpio::{GpioPin, Output, PushPull},
    peripherals::SPI2,
    spi::master::Spi,
    spi::FullDuplexMode,
};
use mipidsi::models::ILI9342CRgb565;

pub struct EmbassyTaskDisplay<'a> {
    pub display: mipidsi::Display<
        SPIInterface<
            Spi<'a, SPI2, FullDuplexMode>,
            GpioPin<Output<PushPull>, 4>,
            GpioPin<Output<PushPull>, 5>,
        >,
        ILI9342CRgb565,
        GpioPin<Output<PushPull>, 48>,
    >,
}

impl DrawTarget for EmbassyTaskDisplay<'static> {
    type Color = Rgb565;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.display.draw_iter(pixels)
    }
}

impl Dimensions for EmbassyTaskDisplay<'static> {
    fn bounding_box(&self) -> Rectangle {
        self.display.bounding_box()
    }
}

impl<'a, 'b> DrawTarget for &'a mut EmbassyTaskDisplay<'b> {
    type Color = Rgb565;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.display.draw_iter(pixels)
    }
}

impl<'a, 'b> Dimensions for &'a mut EmbassyTaskDisplay<'b> {
    fn bounding_box(&self) -> Rectangle {
        self.display.bounding_box()
    }
}
