# ESP32-C3 Bluetooth LE controller blobs

Espressif's Bluetooth LE controller for the ESP32-C3. Most of the controller
lives in the chip's mask ROM; these archives carry the glue around it (the
controller task, the VHCI interface, the OS adapter hookup) and patches that
the ROM reaches through function-pointer tables in RAM.

Contents of this directory:

- `lib/libbtdm_app.a` — the controller. Identical to
  [esp32c3-bt-lib](https://github.com/espressif/esp32c3-bt-lib) `0a08c4b`
  (esp-idf `51d9dfde`).
- `lib/libbtbb.a`, `lib/libphy.a`, `lib/libcoexist.a` — baseband, PHY and
  coexistence libraries the controller depends on.
  All four are taken from the `esp-wifi-sys-esp32c3` 0.3.0 crate, which
  copies them from ESP-IDF.
- `ld/` — the ESP32-C3 ROM linker scripts, from esp-hal's `esp-rom-sys`
  (`ccfe82c`), which takes them from ESP-IDF. The set matches the one esp-hal
  links for the BLE controller: the base ROM, ECO3 and the BT function
  tables. The ECO7 scripts are deliberately absent, as in esp-hal.

These are not linked into a Rust binary: a board links the archives and ROM
scripts into a separate image, passing each ROM script to the linker with
`-T`, and finds this directory next to the crate's `Cargo.toml` (e.g. through
`cargo metadata`).

The binaries and linker scripts are from ESP-IDF, licensed under the Apache
License, Version 2.0.
