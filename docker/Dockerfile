FROM --platform=linux/amd64 nigeleke/docker-dioxus-cli AS builder
WORKDIR /usr/src/app
COPY .. .
RUN dx bundle

FROM nginx:alpine
WORKDIR /usr/share/nginx/html/
COPY --from=builder /usr/src/app/target/dx/scopa/release/web/public/ .
COPY default.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
