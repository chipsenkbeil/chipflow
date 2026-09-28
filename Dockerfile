# Build stage
FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates
COPY static ./static
COPY migrations ./migrations
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/pomodoro-kanban /app/pomodoro-kanban
COPY templates ./templates
COPY static ./static
# migrations are embedded in the binary via sqlx::migrate!
ENV PORT=3000 \
    DATABASE_URL=sqlite:/data/app.db?mode=rwc \
    POMODORO_MINUTES=25
VOLUME /data
EXPOSE 3000
CMD ["/app/pomodoro-kanban"]
