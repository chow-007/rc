rm -f ./go-example/librc.so
rm -f ./go-example/rc.h

cargo build --release
cp target/release/librc.so ./go-example/
cbindgen . --lang c -o  ./go-example/rc.h

cd go-example
go build -o gobin