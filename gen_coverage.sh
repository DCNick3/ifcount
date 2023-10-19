rm -r target/coverage

RUSTFLAGS="-C instrument-coverage" cargo test --tests

grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html

rm *.profraw

echo "coverage report: file://$PWD/target/coverage/html/index.html"
