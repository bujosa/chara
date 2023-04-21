use clap::Arg;

pub fn run() {
    let matches = clap::App::new("First App with Clap")
        .version("1.0")
        .author("Bujosa <davidbujosa@gmail.com>")
        .about("This is a simple app with clap, for learning purposes")
        .arg(
            Arg::with_name("build")
                .short('b')
                .long("build")
                .help("Builds the project"),
        )
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Debugs the project"),
        )
        .get_matches();

    if matches.is_present("debug") {
        println!("Debug mode is on");
    }

    if matches.is_present("build") {
        println!("Build mode is on");
    }
}
