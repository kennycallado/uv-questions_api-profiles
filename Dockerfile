FROM busybox:latest

COPY ./target/x86_64-unknown-linux-musl/release/q-api-profiles /bin/uv-api-profiles
COPY ./Rocket.toml /root/Rocket.toml

WORKDIR /root

CMD ["uv-api-profiles"]

