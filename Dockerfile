FROM debian:bookworm-slim
COPY ./target/release/exporter /app/sensor_exporter
WORKDIR /app
EXPOSE 13000
USER nobody
ENTRYPOINT ["./sensor_exporter"]
