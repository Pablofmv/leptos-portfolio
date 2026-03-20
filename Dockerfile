# ---------- Build ----------
    FROM rust:latest as builder

    WORKDIR /app
    
    RUN cargo install cargo-leptos
    
    COPY . .
    
    # IMPORTANT: enable SSR
    RUN cargo build --release --features ssr
    
    # ---------- Runtime ----------
    FROM debian:bookworm-slim
    
    WORKDIR /app
    
    RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
    
    COPY --from=builder /app/target/release/leptos-portfolio /app/server
    
    EXPOSE 3000
    
    CMD ["./server"]