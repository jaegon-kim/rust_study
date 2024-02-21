# rust_study

## Rust example for study 
https://doc.rust-lang.org/rust-by-example/index.html

Current:
https://doc.rust-lang.org/rust-by-example/std/result.html

## Rust-kr user group
https://rust-kr.org/

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

## References
### Orphan rule
https://rinthel.github.io/rust-lang-book-ko/ch10-02-traits.html
