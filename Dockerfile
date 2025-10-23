FROM ubuntu:latest

RUN apt-get update
# Necessary dependencies for building Tauri apps
RUN apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libudev-dev xdg-utils
# Most scripts assume basic tools are available
RUN apt-get install -y bash curl git build-essential 7zip unzip
# Deno for running TypeScript directly
RUN curl -fsSL https://deno.land/install.sh | bash
ENV PATH="/root/.deno/bin:${PATH}"
# bun.sh for JavaScript/TypeScript runtime
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"
# Rust and Cargo for building Tauri apps
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# Install Tauri CLI
RUN cargo install tauri-cli
# Set working directory
WORKDIR /app
