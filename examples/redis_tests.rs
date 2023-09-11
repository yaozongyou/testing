#![feature(panic_info_message)]

use std::sync::{Mutex, OnceLock};
use testing::testing;
use testing::TestDescAndFn;

pub static REDIS_SERVER_ADDRESS: OnceLock<(String, u16)> = OnceLock::new();

#[testing(comment = "测试set、get、del等基本命令能正常工作")]
fn set_get_del_works() {
    let _address = REDIS_SERVER_ADDRESS.get().unwrap();
    // do some operations on redis at this _address
    // if these operations not meet your expectation, just panic! or assert! to make this test fail
}

#[testing(comment = "lists数据结构相关的测试")]
fn lists_works() {
    let address = REDIS_SERVER_ADDRESS.get().unwrap();

    panic!("the redis {:?} does not work", address);
}

fn main() {
    // setup the test environment
    REDIS_SERVER_ADDRESS
        .set(("127.0.0.1".to_string(), 6380))
        .unwrap();

    for test in inventory::iter::<TestDescAndFn> {
        let test = Mutex::new(Box::new(test));

        let name = test.lock().unwrap().name();
        let comment = test.lock().unwrap().comment();

        println!("begin to run test: {} {}", name, comment);

        std::panic::set_hook(Box::new({
            move |panic_info| {
                let (panic_file, panic_line) = if let Some(location) = panic_info.location() {
                    (location.file(), location.line())
                } else {
                    ("cannot get panic location", 0)
                };

                if let Some(message) = panic_info.message() {
                    println!(
                        "thread 'main' panicked at '{:?}', {}:{}",
                        message, panic_file, panic_line
                    );
                }
            }
        }));
        let result = std::panic::catch_unwind(|| {
            test.lock().unwrap().run();
        });
        let _ = std::panic::take_hook();

        match result {
            Ok(()) => {
                println!("{} success", name);
            }
            Err(_) => {
                println!("{} failure", name);
            }
        }
    }
}
