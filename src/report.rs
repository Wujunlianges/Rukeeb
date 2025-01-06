pub use usbd_human_interface_device::page::{Consumer, Desktop, Keyboard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Report {
    Keyboard(Keyboard),
    Consumer(Consumer),
    Desktop(Desktop),
    Custom(u8),
}
