use clap::App;
use log::debug;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    debug!("ferris_watch starting ...");

    let matches = App::new("ferris_watch")
        .version("0.1.0")
        .author("otofu-square")
        .about("cute watch command")
        .get_matches();

    Ok(())
}
