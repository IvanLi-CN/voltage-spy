use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, mutex::Mutex,
    pubsub::PubSubChannel,
};
use embassy_time::Duration;

use crate::{
    button::ButtonState,
    display::Display,
    types::{Direction, Page, ST7789DCPin, ST7789RstPin, ST7789SpiDev},
};

pub const MIN_PRESS_DURATION: Duration = Duration::from_millis(50);
pub const SHORT_PRESS_DURATION: Duration = Duration::from_millis(200);
pub const DOUBLE_CLICK_TIMEOUT: Duration = Duration::from_millis(300);
pub const MAX_SIMULTANEOUS_PRESS_DELAY: Duration = Duration::from_millis(100);

pub const OCP_MAX: f64 = 10.0;

pub static DISPLAY: Mutex<
    CriticalSectionRawMutex,
    Option<Display<ST7789SpiDev, ST7789DCPin, ST7789RstPin>>,
> = Mutex::new(None);

pub(crate) static BTN_A_STATE_CHANNEL: Channel<CriticalSectionRawMutex, ButtonState, 10> =
    Channel::new();
pub(crate) static BTN_B_STATE_CHANNEL: Channel<CriticalSectionRawMutex, ButtonState, 10> =
    Channel::new();

pub(crate) static PAGE_PUBSUB: PubSubChannel<CriticalSectionRawMutex, Page, 2, 2, 1> =
    PubSubChannel::new();
pub(crate) static BACKLIGHT_PUBSUB: PubSubChannel<CriticalSectionRawMutex, u16, 2, 2, 1> =
    PubSubChannel::new();
pub(crate) static DISPLAY_DIRECTION_PUBSUB: PubSubChannel<
    CriticalSectionRawMutex,
    Direction,
    2,
    2,
    1,
> = PubSubChannel::new();
pub(crate) static OCP_PUBSUB: PubSubChannel<CriticalSectionRawMutex, f64, 2, 2, 1> =
    PubSubChannel::new();
pub(crate) static UVP_PUBSUB: PubSubChannel<CriticalSectionRawMutex, f64, 2, 2, 1> =
    PubSubChannel::new();
pub(crate) static PDO_PUBSUB: PubSubChannel<CriticalSectionRawMutex, f64, 2, 2, 1> =
    PubSubChannel::new();

pub(crate) static PAGE_MUTEX: Mutex<CriticalSectionRawMutex, Page> = Mutex::new(Page::Monitor);
pub(crate) static BACKLIGHT_MUTEX: Mutex<CriticalSectionRawMutex, u16> = Mutex::new(255);
pub(crate) static DISPLAY_DIRECTION_MUTEX: Mutex<CriticalSectionRawMutex, Direction> =
    Mutex::new(Direction::Normal);
pub(crate) static OCP_MUTEX: Mutex<CriticalSectionRawMutex, f64> = Mutex::new(0.0);
pub(crate) static UVP_MUTEX: Mutex<CriticalSectionRawMutex, f64> = Mutex::new(0.0);
pub(crate) static PDO_MUTEX: Mutex<CriticalSectionRawMutex, f64> = Mutex::new(0.0);