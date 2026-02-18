# Embassy for Raspberry Pi Pico 2W

## Getting Started

See: [embassy.dev](https://embassy.dev/book/#_getting_started)

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

Add required Rust target with *hard FPU* support:

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
