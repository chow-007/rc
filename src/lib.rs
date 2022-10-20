use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Rust 中计算的值，并没有返回给 C
#[no_mangle]
pub extern "C" fn print_sum(a: u32, b: u32) {
    let c = a + b;
    println!("print in rust, sum is: {}", c);
}

// Rust 中计算的值，返回给 C
#[no_mangle]
pub extern "C" fn addtwo(a: u32, b: u32) -> u32 {
    let c = a + b;
    c
}

// 其参数为 C 端生成的一个字符串，实现打印和修改该字符串的功能；
#[no_mangle]
pub extern "C" fn print_str(s: *const c_char) {
    let slice = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = slice.to_str().unwrap();
    println!("print in rust, str is: {:?}", r_str);
}

// 其参数为 C 端生成的一个字符串，实现修改该字符串的功能
#[no_mangle]
pub extern "C" fn change_str(s: *mut c_char) -> *mut c_char {
    let mut string = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s).to_string_lossy().into_owned()
    };

    let s = std::fs::read_to_string("./public_suffix_list.dat").expect("read file error");
    string.push_str(s.as_str());
    // println!("Rust side change: {:?}", string);
    let c_str_changed = CString::new(string).unwrap();
    c_str_changed.into_raw()
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

// 其返回值是 Rust 端生成的一个字符串, 需要返回给rust释放内存
#[no_mangle]
pub extern "C" fn generate_str() -> *mut c_char {
    let people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    let mut res = match serde_json::to_string(&people) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };
    let s = std::fs::read_to_string("./public_suffix_list.dat").expect("read file error");
    res.push_str(s.as_str());
    let c_str_song = CString::new(res).unwrap();
    c_str_song.into_raw()
}

// 供 C 端调用者将字符串返回给 Rust 释放内存
#[no_mangle]
pub extern "C" fn free_str(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
