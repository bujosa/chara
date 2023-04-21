use clap::Arg;

pub fn run() {
    let matches = clap::App::new("First App with Clap")
        .version("1.0")
        .author("Bujosa <davidbujosa@gmail.com>")
        .about("This is a simple app with clap, for learning purposes")
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Shows this help message"),
        )
        .get_matches();

    if matches.is_present("help") {
        println!("Help message");
    }
}
