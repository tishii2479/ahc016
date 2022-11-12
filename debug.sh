FILE=$1

cargo build

./tools/target/release/tester ./target/debug/sol < tools/in/$FILE.txt > tools/out/$FILE.txt

pbcopy < tools/out/$FILE.txt