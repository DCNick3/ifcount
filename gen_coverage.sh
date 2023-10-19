rm -r target/coverage

RUSTFLAGS="-C instrument-coverage" cargo test --profile cov --tests

grcov . --binary-path ./target/cov/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html

rm *.profraw crates/**/*.profraw

echo "coverage report: file://$PWD/target/coverage/html/index.html"
