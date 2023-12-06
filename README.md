# rust_study

## Rust example for study 
https://doc.rust-lang.org/rust-by-example/index.html

Current:
https://doc.rust-lang.org/rust-by-example/generics.html

## Run on Containerized Build Environment
<pre>
<code>
make build-env
make
make run
</code>
</pre>
  
## Run on Local Build Environment
<pre>
<code>
sudo apt update
sudo apt install -y curl gcc make build-essential
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
make local
make local-run
</code>
</pre>
  
