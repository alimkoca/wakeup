pub mod timer {
    use std::fs::File;
    use chrono::{Local, NaiveTime, DateTime, Timelike};
    use unicode_segmentation::UnicodeSegmentation;

    /*
     * AlarmObject:
     *      alarm: Naive Time with HH:MM format as Option
     *      cache: Cache file for writing back
     *      home: Home directory for keeping cache against
     *      any damage or losing
     */
    pub struct AlarmObject {
        pub alarm: Option<NaiveTime>,
        pub cache: Option<File>,
        pub home: String
    }

    /* Set AlarmObject structure for keeping data
     * with more reliable solution
     */
    pub fn set_alarm(time: &String) -> Option<NaiveTime> {
        if time.chars().nth(2).unwrap() != ':' && time.chars().nth(2).unwrap() != '.' {
            println!("Wrong alarm format!");
            return None;
        }

        if time.graphemes(true).count() != 5 {
            println!("Wrong alarm format!");
            return None;
        }

        let time_dtf = NaiveTime::parse_from_str(time, "%H:%M");

        return Some(time_dtf.unwrap());
    }

    pub fn getdaytime() -> DateTime<Local> {
        Local::now()
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
