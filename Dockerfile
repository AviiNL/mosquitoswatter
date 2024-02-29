# Build stage
FROM rust:alpine AS build
WORKDIR /app
COPY . .
RUN apk update && apk add --no-cache musl-dev
RUN cargo build --release

# Deploy stage
FROM alpine
WORKDIR /app
COPY --from=build /app/target/release/mosquito-swatter .
CMD ["./mosquito-swatter"]
