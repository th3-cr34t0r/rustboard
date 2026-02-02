use const_gen::CompileConst;
use serde::Deserialize;

#[derive(Deserialize, Debug, CompileConst)]
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

#[derive(Deserialize, Debug)]
pub struct BleConfig {
    pub name: String,
    pub split: bool,
}

#[derive(Deserialize, Debug)]
pub struct MatrixConfig {
    pub row_pins: Vec<Pins>,
    pub col_pins: Vec<Pins>,
}

#[derive(Deserialize, Debug)]
pub struct DebounceConfig {
    pub key_debounce: u64,
}

#[derive(Deserialize, Debug)]
pub struct KeymapConfig {
    pub layers: usize,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ble: BleConfig,
    pub matrix: MatrixConfig,
    pub debounce: Option<DebounceConfig>,
    pub keymap: KeymapConfig,
}
