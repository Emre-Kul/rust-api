# rust-nickel-api
This project created for benchmarking purposes
## INSTALL AND RUN
Project port is 6767
### CARGO
```
cargo install --path .
cargo run
```
### DOCKER
```
docker build -t rust-nickel-api .
docker run --rm -it -p 6767:6767 rust-nickel-api
```