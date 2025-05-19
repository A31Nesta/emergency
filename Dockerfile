FROM rust:slim AS build-env
WORKDIR /emg
COPY . /emg
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /emg/target/release/emergency /
EXPOSE 3000
CMD ["./emergency"]
