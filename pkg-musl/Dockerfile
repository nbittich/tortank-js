FROM node:16-alpine
RUN wget https://sh.rustup.rs -O - | sh -s -- -y
RUN ln -s "${HOME}"/.cargo/bin/* /usr/local/bin
# hack to get dynlibs working with musl
# https://github.com/rust-lang/rust/pull/55163#issuecomment-436631090
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache build-base musl-dev 
COPY . /app

WORKDIR /app
RUN npm i
CMD ["sh", "$1"]
