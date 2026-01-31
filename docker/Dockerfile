FROM --platform=linux/amd64 nigeleke/docker-dioxus-cli AS builder
WORKDIR /usr/src/app
COPY .. .
RUN dx bundle --platform=web --release

FROM nginx:alpine
WORKDIR /usr/share/nginx/html/

RUN chown -R nginx:nginx /usr/share/nginx/html
USER nginx

COPY --from=builder /usr/src/app/target/dx/scopa/release/web/public/ .
EXPOSE 80
