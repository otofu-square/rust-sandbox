use clap::{value_t, App, Arg};
use log::debug;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    debug!("ferris_watch starting ...");

    let window = pancurses::initscr();
    struct EndWin;
    impl Drop for EndWin {
        fn drop(&mut self) {
            pancurses::endwin();
        }
    };
    let _endwin = EndWin;

    'outer: loop {
        let interrupted = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::SIGINT, interrupted.clone())?;
        let interrupted = || interrupted.load(Ordering::SeqCst);

        let matches = App::new("ferris_watch")
            .version("0.1.0")
            .author("otofu-square")
            .about("cute watch command")
            .arg(
                Arg::with_name("command")
                    .required(true)
                    .multiple(true)
                    .help("The command to run periodically"),
            )
            .arg(
                Arg::with_name("interval")
                    .long("interval")
                    .short("n")
                    .takes_value(true)
                    .default_value("2.0")
                    .help("The period to run a command"),
            )
            .get_matches();

        let command = matches.values_of("command").unwrap().collect::<Vec<_>>();
        let interval = value_t!(matches, "interval", f64)?;

        let output = Command::new(command[0]).args(&command[1..]).output()?;
        let output = String::from_utf8_lossy(&output.stdout);

        window.clear();
        window.printw(output);
        window.refresh();

        let interval10 = (interval * 10.0) as u32;

        // for に入る前に interrupted のフラグが立った場合
        if interrupted() {
            break 'outer;
        }

        for _ in 0..interval10 {
            sleep(Duration::from_millis(100));
            if interrupted() {
                break 'outer;
            }
        }
    }

    debug!("end");
    Ok(())
}
