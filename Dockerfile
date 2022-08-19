FROM ubuntu:22.04 as build
LABEL maintainer="{{authors}}"

ENV CARGO_HOME=/usr/local/cargo \
    DEBIAN_FRONTEND=noninteractive \
    PATH=/usr/local/cargo/bin:$PATH \
    RUSTUP_HOME=/usr/local/rustup \
    RUSTUP_VERSION=1.25.1 \
    RUST_VERSION=1.63

RUN apt-get update
RUN apt-get install -y \
	wget \
	build-essential \
	git \
	musl-dev \
        musl-tools

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='5cc9ffd1026e82e7fb2eec2121ad71f4b0f044e88bca39207b3f6b769aaa799c' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/${rustArch}/rustup-init"; \
    wget --progress=dot:giga "$url"; \
    sha256sum rustup-init; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src
COPY . .

RUN cargo install --path . --target x86_64-unknown-linux-musl

FROM scratch
LABEL maintainer="{{authors}}"

COPY --from=build /usr/local/cargo/bin/{{project-name}} /usr/local/bin/{{project-name}}

CMD ["{{project-name}}", "-v"]
