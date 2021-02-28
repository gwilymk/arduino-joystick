FROM rustembedded/cross:armv7-unknown-linux-gnueabihf

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes libudev-dev:armhf

ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig