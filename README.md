# RUSTBOARD

- Rust based firmware for split keyboards

## About:
- Initially started as an ESP32 project, being build for fun and learning embedded rust.

## Features:
- Supporting bluetooth
- Supporting split keyboards
- Layers
- Combos

## Current bugs:
- Unable to remember paired devices

## How to compile:
- cargo build --release --features central / peripheral

### To build uf2 firmware:
- cargo make uf2 --release

- The above command will generate 2 .uf2 file, one peripheral one central, that can be drag and dropped to the bootloader

## TODO:
- [ ] Central connection to be improved - (kinda improved it, need to turn on the central split, then the peripheral in order to connect correctly)
- [ ] Improve central device connection (scan for avalible devices, check for vendor id, name, charactersitics that match the peripheral, then connect (no specifying of the peripherals ble address))
- [ ] Introduce macros feature
- [ ] Introduce sleep
- [ ] Finish up implementing user_config.toml configuration
- [ ] Make esp32 compatible
- [ ] Write detailed documentation on how to set up
- [ ] UI for configuration?
- [ ] Clear stored BLE pairing informtaion on a key combo / keypress for several seconds
- [ ] Improve combos feature setup

- [X] ~~Improve latency~~
- [X] ~~Share central battery level with peripheral, show the lower value to the connected device~~ - done (although on samo nrf52 clones, looks like the pin is not the correct one)
- [X] ~~Enter bootloader more easily~~ - bootloader is entered when key row:0, col:0 is held and released after 5s
- [X]  ~~Introduce combos feature~~ - done 
- [X]  ~~Solder battery and a power-switch~~ - done 
- [X]  ~~Battery level readings (saadc) feature~~ - done
- [X]  ~~Introduce a debounce feature for the matrix (sometimes with the current approach, some keys are registered 2 times)~~ - done
