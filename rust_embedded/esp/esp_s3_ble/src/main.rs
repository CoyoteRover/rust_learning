use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::sys::link_patches;
// 注意：引入了 BLEAdvertisementData
use esp32_nimble::{uuid128, BLEAdvertisementData, BLEDevice, NimbleProperties};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// === LED 相关引用 ===
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let pins = peripherals.pins;

    println!("=== ESP32-S3 NimBLE v0.11 启动 ===");

    // ============================================
    // 1. 初始化 LED (v0.13.1 正确版)
    // ============================================

    // 直接把 "通道" 和 "引脚" 传进去，库内部会自动创建驱动
    let led_driver = Ws2812Esp32Rmt::new(peripherals.rmt.channel0, pins.gpio48)?;

    // 依然用 Arc<Mutex> 包裹
    let led_shared = Arc::new(Mutex::new(led_driver));

    // 下面的 set_color 和之前的逻辑保持不变...
    let led_clone = led_shared.clone();
    let set_led_color = move |r, g, b| {
        let mut led = led_clone.lock().unwrap();
        // 注意：v0.13.1 内部通常已经处理了 GRB 顺序
        // 如果发现颜色不对（比如红绿反了），我们再在这里手动调换
        let _ = led.write(std::iter::once(RGB8::new(r, g, b)));
    };
    // 刚启动：亮蓝色
    set_led_color(0, 0, 20);

    // 1. 初始化设备
    let device = BLEDevice::take();
    let server = device.get_server();

    // 回调中的 LED 控制
    let led_on_connect = led_shared.clone();
    let led_on_disconnect = led_shared.clone();

    // 2. 连接回调
    server.on_connect(move |server, desc| {
        println!(">>> 设备已连接: {:?}", desc.address());

        let mut led = led_on_connect.lock().unwrap();
        // 变绿色
        let _ = led.write(std::iter::once(RGB8::new(0, 20, 0)));

        // 尝试更新连接参数
        if let Err(e) = server.update_conn_params(desc.conn_handle(), 6, 12, 0, 100) {
            println!("更新参数提示: {:?}", e);
        }
    });

    server.on_disconnect(move |_, reason| {
        println!("<<< 手机已断开，原因: {:?}", reason);
        let mut led = led_on_disconnect.lock().unwrap();
        // 变回蓝色
        let _ = led.write(std::iter::once(RGB8::new(0, 0, 20)));
    });

    // 3. 创建服务
    // 修复点：使用 uuid128! 宏
    let service_uuid = uuid128!("0000AAAA-0000-1000-8000-00805F9B34FB");
    let service = server.create_service(service_uuid);

    // 4. 创建特征值
    // 修复点：使用 uuid128! 宏
    let char_uuid = uuid128!("0000BBBB-0000-1000-8000-00805F9B34FB");
    let characteristic = service.lock().create_characteristic(
        char_uuid,
        NimbleProperties::READ | NimbleProperties::WRITE | NimbleProperties::NOTIFY,
    );

    characteristic.lock().set_value(b"Hello NimBLE 0.11!");

    let led_for_data = led_shared.clone();

    characteristic.lock().on_write(move |args| {
        let data = args.recv_data();
        let msg = String::from_utf8_lossy(data);
        println!("[v0.11 收到]: {}", msg);

        // 收到数据闪烁红色
        let mut led = led_for_data.lock().unwrap();
        let _ = led.write(std::iter::once(RGB8::new(20, 0, 0)));
    });

    // 5. 配置广播 (v0.11 新写法)
    let ble_advertising = device.get_advertising();

    // 创建广播数据包
    let mut ad_data = BLEAdvertisementData::new();
    ad_data.name("S3-BLE-0.11"); // 设置广播名
    ad_data.add_service_uuid(service_uuid); // 添加服务UUID让手机能扫到

    // 将数据包设置给广播器
    ble_advertising.lock().set_data(&mut ad_data)?;

    // 启动广播
    match ble_advertising.lock().start() {
        Ok(_) => println!(">>> 广播已启动，名称: 'S3-BLE-0.11'"),
        Err(e) => println!("!!! 广播启动失败: {:?}", e),
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
