# RUST FFI编程


rust导出共享库，供其他语言使用

运行cbuild.sh,导出rust c共享库，并且编译c二进制cbin

运行gobuild.sh脚本，导出rust c共享库，并且编译go二进制文件gobin


查看内存变化
```bash
while true; do ps -ef | grep cbin | grep -v grep | awk '{print $2}' | xargs -I{} cat /proc/{}/status | grep VmHWM; echo ""; sleep 1; done;
```