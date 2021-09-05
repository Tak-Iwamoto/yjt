use clap::{crate_version, App, Arg};

pub fn build_cli() -> App<'static> {
    App::new("yjt")
        .version(crate_version!())
        .arg(
            Arg::new("filetype")
                .long("filetype")
                .short('f')
                .takes_value(true),
        )
        .arg(Arg::new("input").index(1))
}
