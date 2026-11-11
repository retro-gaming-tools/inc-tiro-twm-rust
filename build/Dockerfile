# The release build: Rust for Linux and for Windows (MinGW), SDL3 for both,
# and appimagetool. Every download is pinned and checked by hash, so anyone
# building this gets the same tools.
FROM rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0

ENV CARGO_HOME=/opt/cargo \
    SDL_VERSION=3.4.18 \
    SDL_SHA256=9c75cf16330322c217dedd2e0609f1124f1b54b8633e763467b4684d0f4334a3 \
    APPIMAGETOOL_VERSION=1.9.1 \
    APPIMAGETOOL_SHA256=ed4ce84f0d9caff66f50bcca6ff6f35aae54ce8135408b3fa33abfc3cb384eb0 \
    RUNTIME_VERSION=20251108 \
    RUNTIME_SHA256=2fca8b443c92510f1483a883f60061ad09b46b978b2631c807cd873a47ec260d \
    APPIMAGE_EXTRACT_AND_RUN=1 \
    ARCH=x86_64

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64 cmake ninja-build pkg-config zip file jq \
        libasound2-dev libpulse-dev libpipewire-0.3-dev libdbus-1-dev libudev-dev \
        libx11-dev libxext-dev libxrandr-dev libxcursor-dev libxi-dev libxss-dev libxfixes-dev \
        libxtst-dev libwayland-dev libxkbcommon-dev libegl1-mesa-dev libgl1-mesa-dev \
        libgles2-mesa-dev libdrm-dev libgbm-dev libvulkan-dev; \
    rm -rf /var/lib/apt/lists/*; \
    rustup target add x86_64-pc-windows-gnu; \
    curl -fsSL -o /tmp/sdl.tar.gz "https://github.com/libsdl-org/SDL/releases/download/release-${SDL_VERSION}/SDL3-${SDL_VERSION}.tar.gz"; \
    echo "${SDL_SHA256}  /tmp/sdl.tar.gz" | sha256sum -c -; \
    tar xzf /tmp/sdl.tar.gz -C /tmp; \
    cmake -S "/tmp/SDL3-${SDL_VERSION}" -B /tmp/sdl-linux -G Ninja \
        -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/opt/sdl3 -DCMAKE_INSTALL_LIBDIR=lib \
        -DSDL_SHARED=ON -DSDL_STATIC=OFF -DSDL_TESTS=OFF -DSDL_EXAMPLES=OFF; \
    cmake --build /tmp/sdl-linux; \
    cmake --install /tmp/sdl-linux; \
    cmake -S "/tmp/SDL3-${SDL_VERSION}" -B /tmp/sdl-windows -G Ninja \
        -DCMAKE_SYSTEM_NAME=Windows -DCMAKE_C_COMPILER=x86_64-w64-mingw32-gcc \
        -DCMAKE_CXX_COMPILER=x86_64-w64-mingw32-g++ -DCMAKE_RC_COMPILER=x86_64-w64-mingw32-windres \
        -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/opt/sdl3-windows \
        -DSDL_SHARED=ON -DSDL_STATIC=OFF -DSDL_TESTS=OFF -DSDL_EXAMPLES=OFF; \
    cmake --build /tmp/sdl-windows; \
    cmake --install /tmp/sdl-windows; \
    rm -rf /tmp/sdl.tar.gz "/tmp/SDL3-${SDL_VERSION}" /tmp/sdl-linux /tmp/sdl-windows; \
    curl -fsSL -o /usr/local/bin/appimagetool "https://github.com/AppImage/appimagetool/releases/download/${APPIMAGETOOL_VERSION}/appimagetool-x86_64.AppImage"; \
    echo "${APPIMAGETOOL_SHA256}  /usr/local/bin/appimagetool" | sha256sum -c -; \
    curl -fsSL -o /opt/appimage-runtime "https://github.com/AppImage/type2-runtime/releases/download/${RUNTIME_VERSION}/runtime-x86_64"; \
    echo "${RUNTIME_SHA256}  /opt/appimage-runtime" | sha256sum -c -; \
    chmod +x /usr/local/bin/appimagetool; \
    mkdir -p /opt/cargo/registry; \
    chmod -R a+rwX /opt/cargo
