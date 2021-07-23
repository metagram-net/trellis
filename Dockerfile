FROM rust:latest

RUN mkdir /tmp/node \
    && cd /tmp/node \
    && curl -fsSL https://deb.nodesource.com/setup_16.x > setup.sh \
    && echo '22e2048e76f0b0dd5e126cb816e59665ca4b402ffe9bb542005ad661b36a9eb6  setup.sh' > SHA256SUMS \
    && sha256sum -c SHA256SUMS \
    && bash setup.sh \
    && apt-get install -y nodejs

RUN mkdir /tmp/wasm-pack \
    && cd /tmp/wasm-pack \
    && curl -fsSL https://rustwasm.github.io/wasm-pack/installer/init.sh > init.sh \
    && echo 'befb0a41dc1c6816233b2f22d6f23090900b42d44ff6d8950ea75812ac5a31cf  init.sh' > SHA256SUMS \
    && sha256sum -c SHA256SUMS \
    && sh /tmp/wasm-pack/init.sh

WORKDIR /app
COPY . .

RUN make release && rm -rf node_modules pkg target
