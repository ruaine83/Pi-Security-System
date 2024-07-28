
SVC_NAME=security-pi
BASE_IMG=ubuntu

build_rust_linux:
	@ echo "Building ${SVC_NAME} with Rust for PC Linux"
	@ cargo build

build_rust_pi:
	@ echo "Building ${SVC_NAME} with Rust for R-Pi"
	@ cargo build --target aarch64-unknown-linux-gnu
