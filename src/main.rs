/*
 * @Author: QyInvoLing
 * @Date: 2023-03-09 14:24:49
 * @LastEditors: QyInvoLing
 * @LastEditTime: 2023-03-21 17:39:41
 * @Description:
 */

extern crate winapi;

use std::mem;
use std::thread::sleep;
use std::time::Duration;
use winapi::um::winuser::{GetAsyncKeyState, SendInput, INPUT, INPUT_MOUSE, VK_LBUTTON};

fn main() {
    let mut counter = 0;
    loop {
        unsafe {
            let key_pressed = GetAsyncKeyState(VK_LBUTTON as i32) != 0;
            if key_pressed {
                counter += 1;
            } else {
                counter = 0;
            }
            if counter > 40 {
                let mut input = INPUT {
                    type_: INPUT_MOUSE,
                    u: mem::zeroed(),
                };
                input.u.mi_mut().dx = 2000;
                input.u.mi_mut().dy = 0;
                input.u.mi_mut().dwFlags = winapi::um::winuser::MOUSEEVENTF_MOVE;
                SendInput(1, &mut input as *mut INPUT, mem::size_of::<INPUT>() as i32);
            }
        }
        sleep(Duration::from_millis(1));
    }
}
