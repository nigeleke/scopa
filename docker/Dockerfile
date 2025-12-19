FROM --platform=linux/amd64 nigeleke/docker-dioxus-cli AS builder
WORKDIR /usr/src/app
COPY .. .
RUN dx bundle --release

FROM nginx:alpine
RUN apk add --no-cache \
    avahi \
    avahi-tools \
    dbus \
    dumb-init

WORKDIR /usr/share/nginx/html/

RUN echo "scopa" > /etc/hostname

RUN mkdir -p /etc/avahi
COPY --from=builder /usr/src/app/docker/avahi-daemon.conf /etc/avahi/avahi-daemon.conf
COPY --from=builder /usr/src/app/docker/default.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /usr/src/app/docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

COPY --from=builder /usr/src/app/target/dx/scopa/release/web/public/ .

EXPOSE 80

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["/entrypoint.sh"]
