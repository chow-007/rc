rm -f ./c-example/librc.so
rm -f ./c-example/rc.h

cargo build --release
cp target/release/librc.so ./c-example/
cbindgen . --lang c -o  ./c-example/rc.h

cd ./c-example
gcc -o cbin a.c -L ./ -lrc
