#![no_std]
#![no_main]
#![allow(warnings)]

use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::Rgb565, prelude::*, text::Text, Drawable,
    primitives::{ RoundedRectangle, PrimitiveStyleBuilder, Rectangle },
};
use profont::PROFONT_18_POINT;

use core::fmt::Write as FmtWrite;

fn overlay<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(5)
            .stroke_color(Rgb565::BLACK)
            .fill_color(Rgb565::WHITE)
            .build();

        RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::new(19, 20), Size::new(280, 200)),
            Size::new(10, 10),
        )
        .into_styled(style)
        .draw(display);
}

fn field<D>(display: &mut D, pos: i32, iaq: i32)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

        let color;

        if iaq < 0 {
            color = Rgb565::CSS_ALICE_BLUE; 
        } else {
            color = Rgb565::CSS_LAWN_GREEN;
        }

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(5)
            .stroke_color(Rgb565::BLACK)
            .fill_color(color)
            .build();

        RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::new(220, pos), Size::new(67, 35)),
            Size::new(10, 10),
        )
        .into_styled(style)
        .draw(display);


        
}

fn draw_label<D>(display: &mut D, label: &str, pos_y: i32)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

        let style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);

        Text::new(label, Point::new(35, pos_y), style)
            .draw(display);
}

pub fn build_ui<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

        overlay(display);

        for pos in (30..125).step_by(47) {
            field(display, pos, -1);
        }

        field(display, 171, 10);

        let labels = ["Temperature(°C)", "Humidity(%)", "Pressure(hPa)", "Air quality"];

        let mut l = 0;
        let mut pos_y = 52;
        while pos_y < 203 {
            draw_label(display, labels[l], pos_y);

            l += 1;
            pos_y += 48;
        }
}

pub fn update_data<D>(display: &mut D, sensor_data: f32, pos_y: i32, offset: i32)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

        let text_style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);

        let position: Point = Point::new(229, pos_y);
        let mut data: heapless::String<16> = heapless::String::new();
        write!(data,"{:.2}", sensor_data).unwrap();
        let mut clear_string = heapless::String::<16>::new();
        for _ in 0..data.len() {
            clear_string.push(' ').unwrap_or_default();
        }
        // By redrawing the field, we clear the data
        field(display, pos_y-offset, -1);

        // Draw the new data
        Text::new(
            &data, 
            position, 
            text_style
        )
        .draw(display);
}
