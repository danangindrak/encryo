# Stage 1: Build GLIBC
FROM debian:bullseye-slim AS glibc-builder

# Install dependensi untuk membangun GLIBC
RUN apt-get update && apt-get install -y \
    wget \
    gcc \
    build-essential \
    libc6-dev \
    make \
    gawk \
    bison \
    python3

# Download dan build GLIBC versi 2.29
RUN wget http://ftp.gnu.org/gnu/libc/glibc-2.29.tar.gz && \
    tar -xvzf glibc-2.29.tar.gz && \
    cd glibc-2.29 && \
    mkdir build && cd build && \
    ../configure --prefix=/usr && \
    make -j$(nproc) && \
    make install

# Stage 2: Build aplikasi Rust
FROM rust:1.72 AS builder

# Set working directory
WORKDIR /usr/src/app

# Salin file Cargo.toml dan Cargo.lock untuk caching dependency
COPY Cargo.toml Cargo.lock ./

# Salin semua source code
COPY . .

# Build aplikasi dalam mode release
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bullseye-slim

# Copy GLIBC dari stage `glibc-builder`
COPY --from=glibc-builder /usr/lib /usr/lib
COPY --from=glibc-builder /usr/include /usr/include
COPY --from=glibc-builder /usr/bin /usr/bin

# Install library runtime yang diperlukan
RUN apt-get update && apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Salin binary dari tahap build aplikasi
COPY --from=builder /usr/src/app/target/release/encryo /app/encryo

# Ekspos port aplikasi
EXPOSE 3030

# Jalankan aplikasi
CMD ["/app/encryo"]
