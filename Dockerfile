# ---------- Build ----------
    FROM rust:latest AS builder

    WORKDIR /app
    
    RUN cargo install cargo-leptos
    
    RUN rustup target add wasm32-unknown-unknown
    
    COPY . .
    
    RUN cargo leptos build --release
    
    # ---------- Runtime ----------
    FROM debian:bookworm-slim
    
    WORKDIR /app
    
    RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
    

    COPY --from=builder /app/target/release/leptos-portfolio /app/server
    
    COPY --from=builder /app/target/site /app/target/site
    COPY --from=builder /app/public /app/public
    
    EXPOSE 3000
    
    CMD ["./server"]