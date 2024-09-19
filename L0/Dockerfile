# Build stage
FROM rust:1.80-bullseye as builder

WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release


# Production stage
FROM debian:bullseye
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/l0 .

CMD ["./l0"]