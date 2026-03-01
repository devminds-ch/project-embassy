FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie

# Trigger rustup to install the toolchain from rust-toolchain.toml
COPY rust-toolchain.toml /tmp/rust-toolchain.toml
RUN cd /tmp \
	&& rustc --version \
	&& rm -f /tmp/rust-toolchain.toml

# Install probe-rs for embedded debugging and flashing
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
# Install probe-rs udev rules for USB device access
RUN mkdir -p /etc/udev/rules.d && \
	curl --proto '=https' --tlsv1.2 -LsSf https://probe.rs/files/69-probe-rs.rules -o /etc/udev/rules.d/69-probe-rs.rules
# Add vscode user to plugdev group for USB device access
RUN usermod -aG plugdev vscode
