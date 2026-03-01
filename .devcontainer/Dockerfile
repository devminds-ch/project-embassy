FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie

# Install probe-rs for embedded debugging and flashing
# HACK: until probe-rs 0.32.0 is released we install the latest version from GitHub
# RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
USER vscode
RUN cargo install probe-rs-tools --git https://github.com/probe-rs/probe-rs --locked
USER root
# Install probe-rs udev rules for USB device access
RUN mkdir -p /etc/udev/rules.d && \
	curl --proto '=https' --tlsv1.2 -LsSf https://probe.rs/files/69-probe-rs.rules -o /etc/udev/rules.d/69-probe-rs.rules
# Add vscode user to plugdev group for USB device access
RUN usermod -aG plugdev vscode

# Trigger rustup to install the toolchain from rust-toolchain.toml
COPY rust-toolchain.toml /tmp/rust-toolchain.toml
RUN cd /tmp \
	&& rustc --version \
	&& rm -f /tmp/rust-toolchain.toml
