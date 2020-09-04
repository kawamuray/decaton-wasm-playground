use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::net::TcpStream;

const TASK_BUF_SIZE: usize = 1024 * 1024;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    path: String,
}

#[link(wasm_import_module = "decaton")]
extern "C" {
    fn poll_task(buf_addr: i64, buf_len: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn run() {
    let mut task_buf = [0u8; TASK_BUF_SIZE];
    let task_len = poll_task(task_buf.as_mut_ptr() as i64, task_buf.len() as i32);
    let task: Task = serde_json::from_slice(&task_buf[..task_len as usize]).unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    write!(stream, "GET {} HTTP/1.0\r\n\r\n", task.path).unwrap();
    io::copy(&mut stream, &mut io::stdout()).unwrap();
}
