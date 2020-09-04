use libc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use wasi;

const TASK_BUF_SIZE: usize = 1024 * 1024;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    message: String,
}

#[link(wasm_import_module = "decaton")]
extern "C" {
    fn poll_task(buf_addr: i64, buf_len: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn _initialize() {
    let mut fd = 3;
    while let Ok(stat) = wasi::fd_prestat_get(fd) {
        match stat.tag {
            wasi::PREOPENTYPE_DIR => {
                let prefix_len = stat.u.dir.pr_name_len;
                let mut prefix = vec![0; prefix_len + 1];
                wasi::fd_prestat_dir_name(fd, prefix.as_mut_ptr(), prefix_len).unwrap();
                prefix[prefix_len] = '\0' as u8;
                libc::__wasilibc_register_preopened_fd(
                    fd as i32,
                    Box::into_raw(prefix.into_boxed_slice()) as *const i8,
                );
            }
            _ => break,
        }
        fd += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn run() {
    let mut task_buf = [0u8; TASK_BUF_SIZE];
    let task_len = poll_task(task_buf.as_mut_ptr() as i64, task_buf.len() as i32);
    let task: Task = serde_json::from_slice(&task_buf[..task_len as usize]).unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/messages")
        .unwrap();
    writeln!(file, "{}", task.message).unwrap();
}
