pub mod timer {
    use std::fs::File;
    use std::io::Read;
    use chrono::{Local, NaiveTime, DateTime, Timelike};
    use unicode_segmentation::UnicodeSegmentation;

    /*
     * AlarmObject:
     *      alarm: Naive Time with HH:MM format as Option
     *      log: Log file for writing back
     *      home: Home directory for keeping log against
     *      any damage or losing
     */
    pub struct AlarmObject {
        pub alarm: Option<NaiveTime>,
        pub log: Option<File>,
        pub home: String
    }

    /* Set AlarmObject structure for keeping data
     * with more reliable solution
     */
    pub fn set_alarm(time: &mut String) -> Option<NaiveTime> {
        if time.is_empty() {
            *time = get_time_from();
            assert_eq!(time.pop(), Some('\n'));
            println!("Alarm set for: {time}");

            if time.is_empty() {
                println!("wakeup: wrong alarm format");
                return None;
            }
        }

        if time.graphemes(true).count() != 5 {
            *time = get_time_from();
            assert_eq!(time.pop(), Some('\n'));
            println!("Alarm set for: {time}");

            if time.is_empty() {
                println!("wakeup: wrong alarm format");
                return None;
            }
        }

        if time.chars().nth(2).unwrap() != ':' && time.chars().nth(2).unwrap() != '.' {
            println!("wakeup: wrong alarm format!");
            return None;
        }

        let time_dtf = NaiveTime::parse_from_str(time, "%H:%M")
            .expect("wakeup: couldn't parse time, wrong alarm format");

        return Some(time_dtf);
    }

    pub fn getdaytime() -> DateTime<Local> {
        Local::now()
    }

    pub fn get_time_from() -> String {
        let mut time_from = String::new();
        let time_home_path = format!("{}/.wkup_time", home::home_dir()
                                     .expect("wakeup: couldn't find your home directory")
                                     .display());
        File::open(time_home_path)
            .expect("wakeup: couldn't create or find file")
            .read_to_string(&mut time_from)
            .expect("wakeup: couldn't read as string");

        return time_from;
    }

    /* Check AlarmObject in structure, also can write back
     * for logging
     * */
    pub fn check_alarm(time: &AlarmObject) -> bool {
        let daytime = getdaytime();
        let hour = daytime.hour();
        let min = daytime.minute();

        let alarm_time = match time.alarm {
            Some(atime) => atime,
            None => panic!("wakeup: error time is none!")
        };

        if alarm_time.hour() == hour && alarm_time.minute() == min {
            return true;
        }

        return false;
    }
}

pub mod log {
    use home::home_dir;
    use chrono::Timelike;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use crate::timer::AlarmObject;

    fn is_exists(path: &String) -> bool {
        std::fs::metadata(path).is_ok()
    }

    fn return_logfile() -> String {
        let homeofuser: String = home_dir()
            .unwrap()
            .display()
            .to_string();

        return format!("{}/.wkuplog", homeofuser);
    }

    pub fn create_log() -> Option<std::fs::File> {
        let log_home: String = return_logfile();

        if is_exists(&log_home) {
            return None;
        }

        let log_obj = File::create(log_home)
            .unwrap();

        return Some(log_obj);
    }

    pub fn write_log(time: &AlarmObject) {
        let alarm_time = match time.alarm {
            Some(atime) => atime,
            None => panic!("wakeup: error time is none!")
        };

        let log_home: String = return_logfile();
        let mut log_obj = OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_home)
            .expect("Open error!");

        let content_log: String = format!("Alarm set for {}:{}\n", alarm_time.hour(), alarm_time.minute());

        log_obj.write(content_log.as_bytes())
            .expect("Write error!");
    }
}
