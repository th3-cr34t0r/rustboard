use trouble_host::prelude::{
    characteristic::{BATTERY_LEVEL, BATTERY_LEVEL_STATUS},
    *,
};
use usbd_hid::descriptor::{SerializedDescriptor, generator_prelude::*};

/// Custom service for the split device
pub const SPLIT_SERVICE: BluetoothUuid16 = BluetoothUuid16::new(0xff11);

/// Custom characteristics for the split device
pub const SPLIT_REPORT_CH: BluetoothUuid16 = BluetoothUuid16::new(0xff22);
pub const SPLIT_BATTERY_CH: BluetoothUuid16 = BluetoothUuid16::new(0xff33);

/// KeyboardReport describes a report and its companion descriptor that can be
/// used to send keyboard button presses to a host and receive the status of the
/// keyboard LEDs.
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
        (logical_min = 0,) = {
            #[item_settings constant,variable,absolute] reserved=input;
        };
        (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
            #[packed_bits 5] #[item_settings data,variable,absolute] leds=output;
        };
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
            #[item_settings data,array,absolute] keycodes=input;
        };
    }
)]
#[allow(dead_code)]
#[derive(Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct KeyboardReport {
    pub modifier: u8, // ModifierCombination
    pub reserved: u8,
    pub leds: u8, // LedIndicator
    pub keycodes: [u8; 6],
}

#[gatt_server(cccd_table_size = 8, connections_max = 2)]
pub(crate) struct Server {
    pub(crate) battery_service: BatteryService,
    pub(crate) hid_service: HidService,
    pub(crate) split_service: SplitService,
}

#[gatt_service(uuid = service::BATTERY)]
pub(crate) struct BatteryService {
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, name = "battery_level", read, value = "Battery Level")]
    #[characteristic(uuid = BATTERY_LEVEL, read, notify, value = 0)]
    pub(crate) level: u8,
    #[characteristic(uuid = BATTERY_LEVEL_STATUS, write, read, notify)]
    status: bool,
}
#[gatt_service(uuid = service::HUMAN_INTERFACE_DEVICE)]
pub(crate) struct HidService {
    #[characteristic(uuid = "2a4a", read, value = [0x01, 0x01, 0x00, 0x03])]
    pub(crate) hid_info: [u8; 4],
    #[characteristic(uuid = "2a4b", read, value = KeyboardReport::desc().try_into().expect("Failed to convert KeyboardReport to [u8; 67]"))]
    pub(crate) report_map: [u8; 67],
    #[characteristic(uuid = "2a4c", write_without_response)]
    pub(crate) hid_control_point: u8,
    #[characteristic(uuid = "2a4e", read, write_without_response, value = 1)]
    pub(crate) protocol_mode: u8,
    #[descriptor(uuid = "2908", read, value = [0u8, 1u8])]
    #[characteristic(uuid = "2a4d", read, notify)]
    pub(crate) report: [u8; 8],
    #[descriptor(uuid = "2908", read, value = [0u8, 2u8])]
    #[characteristic(uuid = "2a4d", read, write, write_without_response)]
    pub(crate) output_keyboard: [u8; 1],
}

#[gatt_service(uuid = SPLIT_SERVICE)]
pub(crate) struct SplitService {
    #[characteristic(uuid = SPLIT_REPORT_CH, read, notify)]
    pub(crate) registered_keys: [u8; 6],
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, name = "battery_level", read, value = "Battery Level")]
    #[characteristic(uuid = SPLIT_BATTERY_CH, read, notify, value = 0)]
    pub(crate) level: u8,
}
