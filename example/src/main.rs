#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use display_interface_spi::{SPIInterface, SPIInterfaceNoCS};

use embedded_graphics::{
    mono_font::{ascii::FONT_9X15, MonoTextStyleBuilder},
    prelude::{Point, Primitive},
    primitives::{Line, PrimitiveStyle, Triangle},
    text::{Baseline, Text},
};
use embedded_graphics_core::{
    draw_target::DrawTarget, pixelcolor::Rgb565, prelude::RgbColor, Drawable,
};
use panic_semihosting as _;
use st7789_driver::{Orientation, ST7789};
use stm32f1xx_hal::{
    gpio, pac,
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
    timer,
};

#[entry]
fn main() -> ! {
    //初始化和获取外设对象
    // 获取cortex-m 相关的核心外设
    let cp = cortex_m::Peripherals::take().unwrap();
    //获取stm32f1xx_hal硬件外设
    let dp = pac::Peripherals::take().unwrap();
    // 初始化并获取flash和rcc设备的所有权
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    //冻结系统中所有时钟的配置，并将冻结后的频率值存储在“clocks”中
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz())
        .pclk2(72.MHz())
        .freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split();
    let mut gpiob = dp.GPIOB.split();
    let mut gpioa = dp.GPIOA.split();

    let pins: (
        stm32f1xx_hal::gpio::Pin<'B', 13, stm32f1xx_hal::gpio::Alternate>,
        stm32f1xx_hal::gpio::Pin<'B', 14>,
        stm32f1xx_hal::gpio::Pin<'B', 15, stm32f1xx_hal::gpio::Alternate>,
    ) = (
        gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
        gpiob.pb14.into_floating_input(&mut gpiob.crh),
        gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
    );
    let mut afio = dp.AFIO.constrain();
    // let (pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    let dc = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let cs = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let rst = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    // let blk = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let mut blk: gpio::Pin<'A', 10, gpio::Output> =
        gpioa.pa10.into_push_pull_output(&mut gpioa.crh);

    //精度 1us
    // let mut delay = FTimerUs::new(dp.TIM2, &clocks).delay();
    //精度 1ms
    //   let mut delay = FTimerMs::new(dp.TIM2, &clocks).delay();
    // or
    let mut delay = dp.TIM2.delay_us(&clocks);
    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };
    let spi = Spi::spi2(dp.SPI2, pins, spi_mode, 36.MHz(), clocks);
    // let di = SPIInterfaceNoCS::new(spi, dc);
    let di = SPIInterface::new(spi, dc, cs);

    // display interface abstraction from SPI and DC
    // let di = SPIInterfaceNoCS::new(spi, dc);

    // create driver
    let mut display = ST7789::new(di, Some(rst), Some(blk), 135, 240);
    // display.set_scroll_offset(100).unwrap();
    display.set_offset(52, 40);

    // initialize
    display.init(&mut delay).unwrap();
    // set default orientation
    display
        .set_orientation(Orientation::PortraitSwapped)
        .unwrap();
    display.clear(Rgb565::YELLOW).unwrap();
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X15)
        .text_color(Rgb565::BLUE)
        .build();

    Text::with_baseline(
        "Hello world!33333333333 ",
        Point::new(0, 0),
        text_style,
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();

    loop {
        delay.delay_ms(500_u16);
    }
}

// shared-bus = "0.2.4"
// embedded-graphics-core = "0.3.3"
// embedded-graphics = "0.7.1"
// u8g2-fonts = "0.7.1"
// display-interface = "0.4.1"
// display-interface-spi = "0.4.1"
// st7789 = "=0.7.0"
