use std::{thread, time::Duration};

use hidapi::HidApi;

fn main() {
    let api = HidApi::new_without_enumerate().unwrap();
    let (vid, pid) = (0x1462, 0x3FA4);

    loop {
        let Ok(device) = api.open(vid, pid) else {
            println!("Device not found");
            thread::sleep(Duration::from_secs(1));
            continue;
        };

        loop {
            let mut buf = [0u8; 65];
            buf[0] = 0x01; // report id
            buf[1] = 0x35; // header
            buf[2] = 0x38; // get
            buf[3] = 0x30;
            buf[4] = 0x30;
            buf[5] = 0x31;
            buf[6] = 0x31;
            buf[7] = 0x30;
            buf[8] = 0x0d; // footer

            if let Err(err) = device.write(&buf) {
                println!("Error writing: {:?}", err);
                break;
            }

            if let Err(err) = device.read(&mut buf) {
                println!("Error reading: {:?}", err);
                break;
            }

            // button pressed
            if buf[10] == 49 {
                println!("!! BUTTON PRESSED !!!!");
                let mut buf = [0u8; 65];
                buf[0] = 0x01; // report id
                buf[1] = 0x35; // header
                buf[2] = 0x62; // set
                buf[3] = 0x30;
                buf[4] = 0x30;
                buf[5] = 0x35;
                buf[6] = 0x30;
                buf[7] = 0x30;
                buf[8] = 0x30; //
                buf[9] = 0x30; // -- theory: these three bytes encode the display in ascii: 003 for Usbc
                buf[10] = 0x33; //
                buf[11] = 0x0d; // footer

                if let Err(e) = device.write(&buf) {
                    println!("Error writing: {:?}", e);
                    break;
                }
            } else {
                println!("not pressed");
            }

            thread::sleep(Duration::from_secs(1));
        }

        eprintln!("Device disconnected");
        thread::sleep(Duration::from_secs(5));
    }
}
