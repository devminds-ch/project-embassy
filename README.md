# Embassy Training Project by [devminds GmbH](https://devminds.ch)

This [Embassy](https://embassy.dev) project is used for Docker or embedded Rust trainings.

The project contains an Embassy application ... **TODO** ...

**FIXME:**

* Breaktpoints are NOT working in VSCode!
* https://github.com/probe-rs/probe-rs/issues/3702
* Fixed on `probe-rs` master - install using:
  ```bash
  cargo install probe-rs-tools --git https://github.com/probe-rs/probe-rs --locked
  ```
* Wait for release `0.32.0` !!!


## Getting Started

* [embassy.dev](https://embassy.dev/book/#_getting_started)
* [rp235x-project-templates](https://github.com/rp-rs/rp235x-project-template)

Install [probe-rs](https://probe.rs/docs/getting-started/installation/):

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Install shell completions:

```bash
probe-rs complete install
```

Configure [probe-rs](https://probe.rs/docs/getting-started/probe-setup/) `udev` rules:

* Download: https://probe.rs/files/69-probe-rs.rules
* Move file to `/etc/udev/rules.d`
* Reload rules:
  ```bash
  sudo udevadm control --reload
  sudo udevadm trigger
  ```

Add required Rust target with **hard FPU** support:

```bash
rustup target add thumbv8m.main-none-eabihf
```

## Update Raspberry Pi Debug Probe Firmware

We should have the latest version (2.3.0)!

Check current version:

```bash
lsusb -v -d 2e8a:000c | grep bcdDevice
```

Follow the docs for: [Updating the firmware on the Debug Probe](https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html#updating-the-firmware-on-the-debug-probe).
