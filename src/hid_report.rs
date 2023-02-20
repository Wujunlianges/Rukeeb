use usbd_human_interface_device::page::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HIDReport {
    Keyboard(Keyboard),
}
