cargo build --release -v
cp target/release/boxes run
git add -A
git commit -a -m "$1"
git push
