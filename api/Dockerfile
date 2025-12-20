# Stage 1: Build Rust library
FROM rust:1.90-alpine AS rust-builder
RUN apk add --no-cache musl-dev

COPY csvProcessor /app/csvProcessor
WORKDIR /app/csvProcessor
RUN cargo build --release

# Stage 2: Build Go application
FROM golang:1.24-alpine AS build

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

# Copy the built Rust artifacts from the rust-builder stage
# This places the library where the Go cgo directives expect it (csvProcessor/target/release)
COPY --from=rust-builder /app/csvProcessor/target/release ./csvProcessor/target/release

RUN apk add --no-cache make gcc musl-dev && make build-go

# Stage 3: Final image
FROM alpine:latest

WORKDIR /app

COPY --from=build /app/bin/courses-calculator /bin/server

# Install curl for healthcheck
RUN apk --no-cache add curl

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

ENTRYPOINT ["/bin/server"]
