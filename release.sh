ERVER="8080"
RUNTIME_DIR=~/$SERVER/runtime
EXE=rust-web-demo-$SERVER
cd ~/projects/rust-web-demo
git pull

cargo build --release

ps aux | grep "$EXE" | grep -v grep| awk '{print $2}'| xargs kill -9

rm -Rf $RUNTIME_DIR
mkdir $RUNTIME_DIR

cp target/release/rust_web_demo $RUNTIME_DIR/$EXE
cp -R web-root $RUNTIME_DIR/web-root

cd ~/$SERVER/runtime
nohup ./$EXE > ~/$SERVER/output.log 2>&1 &

