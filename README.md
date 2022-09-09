## wakeup: it wants to wake you up...

- Easy for use
- Basic shell usage
- Why not?

## Basic usage guide
```
$ wakeup %HH:%MM [MUSIC-PATH]
$ # Note: %HH:%MM is Unix time format
$ # and [MUSIC-PATH] is path to mp3 or
$ # any wav file you can play.
```
### Note:
If %HH:%MM is wrong or empty, it will
be replaced time in `$HOME/.wkup_time` as
suitable with %HH:%MM time format.

### Example:
```
[alimkoca@fedora src]$ cargo run 12:2 /opt/alarm.mp3
   Compiling wakeup v0.1.0 (/home/alimkoca/Yazılımlar/wakeup)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `/home/alimkoca/Yazılımlar/wakeup/target/debug/wakeup '12:2' /opt/alarm.mp3`
Alarm set for: 13:09
```
