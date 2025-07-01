use evdev::{Device, EventType};
use rand::seq::IndexedRandom;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::thread;

fn main() {
    let mut device = Device::open("/dev/input/event5").expect("Failed to open device");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let stream_handle = std::sync::Arc::new(stream_handle);

    let volume: f32 = 0.8;

    println!(
        "Listening for key presses on {}",
        device.name().unwrap_or("Unknown Device")
    );
    loop {
        if is_paused() {
            thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }
        for event in device.fetch_events().expect("Failed to fetch events") {
            if event.event_type() == EventType::KEY {
                // let key = KeyCode::new(event.code()); // dont care about the key code rn, might add config to ignore modifier keys later
                match event.value() {
                    0 => println!("Key released"),
                    1 => println!("Key pressed"),
                    2 => println!("Key held down"),
                    _ => {}
                }
                if event.value() == 1 {
                    // Only play sound on key press
                    play_sound(stream_handle.clone(), volume);
                }
            }
        }
    }
}

fn is_paused() -> bool {
    std::path::Path::new("/tmp/rsfx_paused").exists()
}


fn play_sound(stream_handle: std::sync::Arc<rodio::OutputStreamHandle>, volume: f32) {
    let sfx_paths: Vec<&str> = vec![
        "/usr/share/sounds/rsfx/sfx0.ogg",
        "/usr/share/sounds/rsfx/sfx1.ogg",
        "/usr/share/sounds/rsfx/sfx2.ogg",
        "/usr/share/sounds/rsfx/sfx3.ogg",
        "/usr/share/sounds/rsfx/sfx4.ogg",
        "/usr/share/sounds/rsfx/sfx5.ogg",
        "/usr/share/sounds/rsfx/sfx6.ogg",
    ];

    let random_sfx = {
        let mut rng = rand::rng();
        sfx_paths.choose(&mut rng).expect("Failed to choose random sound effect").to_string()
    };

    thread::spawn(move || {
        let file = File::open(&random_sfx).expect("Failed to open random sound file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode sound file");
        let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

        sink.set_volume(volume);
        sink.append(source);
        sink.sleep_until_end();
        println!("Sound played successfully");
    });
}
