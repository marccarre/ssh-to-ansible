setup-coverage:
    rustup update
    rustup component add llvm-tools-preview
    cargo install grcov

setup: setup-coverage
    rustup component add clippy
    command -v pre-commit >/dev/null 2>&1 || brew install pre-commit
    pre-commit install
    pre-commit install --hook-type commit-msg
    pre-commit run --all-files

lint:
    pre-commit run --all-files
    cargo clippy --all --all-targets
    cargo clippy --fix

cover:
    mkdir -p target/coverage/html
    CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' cargo build
    CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
    grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/
    grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov
    rm -f *.profraw
    open ./target/coverage/html/index.html
