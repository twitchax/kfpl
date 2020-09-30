FROM rustlang/rust:nightly AS builder
WORKDIR /build

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-gnu

RUN USER=root cargo new kfpl
WORKDIR /build/kfpl

COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY ./src ./src
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Copy the statically-linked binary into a scratch container.
FROM ubuntu
COPY --from=builder /build/kfpl/target/x86_64-unknown-linux-gnu/release/kfpl /usr/local/bin/kfpl
RUN kfpl -y init
RUN rm -rf /var/lib/apt/lists/*

#ENTRYPOINT [ "/entrypoint.sh" ]