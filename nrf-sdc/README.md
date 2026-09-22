# Nordic SoftDevice Controller

The Bluetooth LE controller (SoftDevice Controller, SDC) and the
Multiprotocol Service Layer (MPSL) it runs on, taken from Nordic's
[sdk-nrfxlib](https://github.com/nrfconnect/sdk-nrfxlib) at revision
`2072df2` (`softdevice_controller` rev `3e73449`).

Contents of `resources/`:

- `lib/nrf52/soft-float/` — the archives, copied from nrfxlib's
  `softdevice_controller/lib/nrf52/soft-float/`,
  `mpsl/lib/nrf52/soft-float/` and
  `mpsl/fem/common/lib/nrf52/soft-float/`. These match Rust's
  `thumbv7em-none-eabi`. nrfxlib also ships `hard-float` and
  `softfp-float` sets, which are not vendored here.
- `include/softdevice_controller/`, `include/mpsl/` — the corresponding
  headers, unmodified.
- `include/stub/` — `nrfx.h` and `hal/nrf_clock.h` shims, so the headers
  above parse without a copy of the nRF MDK. Neither affects the layout of
  any type.

Select the controller variant with the `peripheral` (default), `central` or
`multirole` feature.

The binaries and headers are licensed under Nordic's 5-clause license, in
[resources/LICENSE](./resources/LICENSE). Note that clause 4 restricts use to
Nordic Semiconductor integrated circuits, and that clause 5 prohibits reverse
engineering, decompiling, modifying and disassembling the binaries.
