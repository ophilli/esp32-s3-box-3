#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::cell::RefCell;
use critical_section::Mutex;

// display and graphics imports
use display_interface_spi::SPIInterface;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
mod embassy_task_ili9342c;
use embassy_task_ili9342c::EmbassyTaskDisplay;

// esp-box UI elements imports
use esp_box_ui::{
    build_inventory,
    food_item::{update_field, FoodItem},
};

// peripherals imports
use hal::{
    clock::{ClockControl, CpuClock},
    embassy,
    peripherals::Peripherals,
    prelude::{_fugit_RateExtU32, *},
    spi::{master::Spi, SpiMode},
    timer::TimerGroup,
    Delay, IO,
};

// embassy imports
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_println::println;

static HOTDOG: Mutex<RefCell<FoodItem>> = Mutex::new(RefCell::new(FoodItem {
    name: "Hotdog",
    pos_y: 17,
    amount: 10,
    price: 2.50,
    highlighted: false,
    purchased: false,
}));
static SANDWICH: Mutex<RefCell<FoodItem>> = Mutex::new(RefCell::new(FoodItem {
    name: "Sandwich",
    pos_y: 87,
    amount: 9,
    price: 3.50,
    highlighted: false,
    purchased: false,
}));
static ENERGY_DRINK: Mutex<RefCell<FoodItem>> = Mutex::new(RefCell::new(FoodItem {
    name: "Energy Drink",
    pos_y: 157,
    amount: 11,
    price: 2.00,
    highlighted: false,
    purchased: false,
}));

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();

    let timer0 = TimerGroup::new(peripherals.TIMG0, &clocks).timer0;

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    embassy::init(&clocks, timer0);

    let mut delay = Delay::new(&clocks);

    let sclk = io.pins.gpio7;
    let mosi = io.pins.gpio6;

    let cs = io.pins.gpio5.into_push_pull_output();
    let dc = io.pins.gpio4.into_push_pull_output();
    let mut backlight = io.pins.gpio47.into_push_pull_output();
    let mut reset = io.pins.gpio48.into_push_pull_output();

    reset.internal_pull_up(true);

    let spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        // cs,
        40u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    );

    let di = SPIInterface::new(spi, dc, cs);
    // let di = SPIInterfaceNoCS::new(spi, dc);
    delay.delay_ms(500u32);

    let mut display_struct = EmbassyTaskDisplay {
        display: match mipidsi::Builder::ili9342c_rgb565(di)
            .with_display_size(320, 240)
            .with_framebuffer_size(320, 240)
            .with_orientation(mipidsi::Orientation::PortraitInverted(false))
            .with_color_order(mipidsi::ColorOrder::Bgr)
            .init(&mut delay, None)
        {
            Ok(display) => {
                println!("Display initialization succeeded!");
                display
            }
            Err(e) => {
                println!("Display initialization failed: {:?}", e);
                panic!("Display initialization failed");
            }
        },
    };

    backlight.set_high().unwrap();

    println!("pre-clear");
    display_struct.display.clear(Rgb565::WHITE).unwrap();
    println!("post-clear");

    let hotdog = critical_section::with(|cs| HOTDOG.borrow(cs).borrow().clone());
    let sandwich = critical_section::with(|cs| SANDWICH.borrow(cs).borrow().clone());
    let energy_drink = critical_section::with(|cs| ENERGY_DRINK.borrow(cs).borrow().clone());

    build_inventory(
        &mut display_struct.display,
        &hotdog,
        &sandwich,
        &energy_drink,
    );

    update_field(&mut display_struct.display, &hotdog);
    update_field(&mut display_struct.display, &sandwich);
    update_field(&mut display_struct.display, &energy_drink);
}
