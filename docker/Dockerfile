FROM nigeleke/docker-dioxus-cli as builder
WORKDIR /usr/src/app
COPY .. .
RUN dx bundle

FROM nginx:alpine
WORKDIR /usr/share/nginx/html/scopa/app/
COPY --from=builder /usr/src/app/target/dx/scopa/release/web/public/ .
EXPOSE 80
