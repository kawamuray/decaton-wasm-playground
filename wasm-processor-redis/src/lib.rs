use redis::{self, Commands};
use serde::{Deserialize, Serialize};
use serde_json;

const TASK_BUF_SIZE: usize = 1024 * 1024;
static mut REDIS_CLIENT: Option<redis::Client> = None;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    key: String,
    value: i32,
}

#[link(wasm_import_module = "decaton")]
extern "C" {
    fn poll_task(addr: i64, len: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn _initialize() {
    REDIS_CLIENT.replace(redis::Client::open("redis://127.0.0.1/").unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn run() {
    let mut buf = [0u8; TASK_BUF_SIZE];
    let len = poll_task(buf.as_mut_ptr() as i64, buf.len() as i32);
    let task: Task = serde_json::from_slice(&buf[..len as usize]).unwrap();

    let mut con = REDIS_CLIENT.as_ref().unwrap().get_connection().unwrap();
    // throw away the result, just make sure it does not fail
    let _: () = con.set(&task.key, task.value).unwrap();
    let val: i32 = con.get(&task.key).unwrap();
    eprintln!("Store value: {}", val);
}
