# rust_study

* Rust example for study 
https://doc.rust-lang.org/rust-by-example/index.html

Curr: https://doc.rust-lang.org/rust-by-example/fn.html

* Run on Containerized Build Environment
make build-env
make
make run

* Run on Local Build Environment
sudo apt update
sudo apt install -y curl gcc make build-essential
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
make local
make local-run

