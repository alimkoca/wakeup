use std::env;
use soloud::*;
use wakeup::{timer, cache};

fn main() {
    let args: Vec<String> = env::args().collect();

    let alarm = timer::AlarmObject {
        alarm: timer::set_alarm(&args[1]), // Alarm is first argument 
        cache: cache::create_cache(),
        home: home::home_dir()
            .unwrap()
            .display()
            .to_string()
    };

    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();

    wav.load(std::path::Path::new("/opt/alarm.mp3")).unwrap();

    loop {
        if timer::check_alarm(&alarm) {
            sl.play(&wav);
            while sl.voice_count() > 0 {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            break;
        }
    }

    println!("anan: alarm went off");
}
