package main

/*
   #cgo CFLAGS: -I.
   #cgo LDFLAGS: -L. -lrc
   #include <stdio.h>
   #include <rc.h>
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	C.print_sum(1, 2)

	n := C.addtwo(2, 5)
	fmt.Println("print in go, sum is: ", n)

	for {
		// foo()
		bar()
	}

}

func foo() {
	goStr := "beijing"
	// Memory Alloc in OS allocator And String Copy
	// cStr is not managed by go GC
	cStr := C.CString(goStr)
	defer C.free(unsafe.Pointer(cStr))
	p := C.change_str(cStr) // 0x39b2d890，需要转换成go 字符串
	defer C.free(unsafe.Pointer(p))
	// res := C.GoString(p)
	C.GoString(p)
	// fmt.Println(res)
}

func bar() {
	p := C.generate_str()
	defer C.free(unsafe.Pointer(p))
	// res := C.GoString(p)
	C.GoString(p)
	// fmt.Println(res)
}
