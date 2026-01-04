#![no_std]
#![no_main]

// [配套 1] 引入崩溃处理 (对应 panic-probe)
use panic_probe as _;
// [配套 2] 引入日志传输 (对应 defmt-rtt)
use defmt_rtt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // 用defmt打印日志
    defmt::info!("Hello STM32! 程序启动了！");

    let dp = pac::Peripherals::take().unwrap();
    let _rcc = dp.RCC.constrain();
    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        led.toggle();
        defmt::info!("灯翻转了一次"); // 这行日志会在你的终端显示
        cortex_m::asm::delay(8_000_000);
    }
}
