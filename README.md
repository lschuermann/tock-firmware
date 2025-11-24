# External Firmware Crates

This repo contains third-party firmware that is useful for Tock boards.
The firmware may be source code or pre-compiled binary blobs.

Third-party firmware is collected in this repo to avoid having it be included
in the main Tock repository. Some firmware may be quite large, and this avoids
having all Tock developers needing a copy even if they are not using the relevant
board. By leveraging the cargo build system, Tock boards can directly include
the firmware from this repository.
