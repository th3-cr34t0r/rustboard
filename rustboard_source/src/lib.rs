#![no_std]
#![no_main]

pub mod battery;
pub mod ble;
pub mod config;
pub mod key_provision;
pub mod keycodes;
pub mod keymap;
pub mod matrix;
pub mod peripherals;
pub mod storage;

use crate::{config::MATRIX_KEYS_BUFFER, matrix::KeyPos};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};

/// Shared variable between matrix scan and key provision tasks
pub static MATRIX_KEYS_LOCAL: Watch<CriticalSectionRawMutex, [KeyPos; MATRIX_KEYS_BUFFER], 2> =
    Watch::new();

#[cfg(feature = "peripheral")]
use usbd_hid::descriptor::KeyboardReport;

#[cfg(feature = "peripheral")]
/// Shared variable between ble and key provision tasks
pub static KEY_REPORT: Watch<CriticalSectionRawMutex, KeyboardReport, 2> = Watch::new();

#[cfg(feature = "peripheral")]
/// Shared variable between matrix scan and key provision tasks
pub static MATRIX_KEYS_SPLIT: Watch<CriticalSectionRawMutex, [KeyPos; MATRIX_KEYS_BUFFER], 2> =
    Watch::new();

#[cfg(feature = "central")]
/// Shared variable between ble and key provision tasks
pub static MESSAGE_TO_PERI: Watch<CriticalSectionRawMutex, [u8; 6], 2> = Watch::new();

/// Shared variable for battery percentage information
pub static BATTERY_LEVEL: Watch<CriticalSectionRawMutex, u8, 3> = Watch::new();

use embassy_time::{Duration, Timer};

pub async fn delay_ms(delay: u64) {
    let duration = Duration::from_millis(delay);
    Timer::after(duration).await;
}

pub async fn delay_us(delay: u64) {
    let duration = Duration::from_micros(delay);
    Timer::after(duration).await;
}

pub enum Pins {
    // GPIO port 0
    P0_00,
    P0_01,
    P0_02,
    P0_03,
    P0_04,
    P0_05,
    P0_06,
    P0_07,
    P0_08,
    P0_09,
    P0_10,
    P0_11,
    P0_12,
    P0_13,
    P0_14,
    P0_15,
    P0_16,
    P0_17,
    P0_18,
    P0_19,
    P0_20,
    P0_21,
    P0_22,
    P0_23,
    P0_24,
    P0_25,
    P0_26,
    P0_27,
    P0_28,
    P0_29,
    P0_30,
    P0_31,

    // GPIO port 1
    P1_00,
    P1_01,
    P1_02,
    P1_03,
    P1_04,
    P1_05,
    P1_06,
    P1_07,
    P1_08,
    P1_09,
    P1_10,
    P1_11,
    P1_12,
    P1_13,
    P1_14,
    P1_15,
}

// It includes the user_config.toml variables evaluated on compile time
include!(concat!(env!("OUT_DIR"), "/constants.rs"));
