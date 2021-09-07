FROM rust:latest

COPY build /build
RUN /build/node-setup.sh && apt-get install -y nodejs
RUN /build/wasm-pack-init.sh

WORKDIR /app
COPY . .
RUN make release && rm -rf build node_modules pkg target trellis_web/pkg trellis_web/target
