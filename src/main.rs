use std::{thread, time::Duration};

use clap::Parser;
use hidapi::HidApi;

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(long, value_enum)]
    switch_to: InputSource,

    #[clap(long, default_value_t = 0x1462)]
    vendor_id: u16,

    #[clap(long, default_value_t = 0x3FA4)]
    product_id: u16,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum InputSource {
    Dp = 0x32,
    UsbC = 0x33,
}

fn main() {
    let args = Args::parse();
    let api = HidApi::new_without_enumerate().unwrap();

    loop {
        let device = match api.open(args.vendor_id, args.product_id) {
            Ok(device) => device,
            Err(err) => {
                println!("Got error opening device: {:?}", err);
                thread::sleep(Duration::from_secs(1));
                continue;
            }
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
                buf[10] = args.switch_to as u8; //
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
