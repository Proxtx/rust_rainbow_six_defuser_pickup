//#![windows_subsystem = "windows"]

use captrs::*;
use enigo::*;
use shuteye::sleep;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();

    loop {
        let mut capturer = Capturer::new(0).unwrap();

        match capturer.capture_frame() {
            Err(_) => {}
            Ok(_) => {}
        }
        match capturer.capture_frame() {
            Err(_) => {
                println!("Error");
            }
            Ok(ps) => {
                if ps[1974555].r == 251 && ps[1974555].g == 201 && ps[1974555].b == 0 {
                    println!("Trigger");
                    enigo.key_click(enigo::Key::Layout('f'));
                }
            }
        }

        sleep(Duration::from_millis(80));
    }
}
