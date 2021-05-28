use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
  App::new("rusty cow{{say,think}}")
    .version(env!("CARGO_PKG_VERSION"))
    .author("(c) 2021 Nirugiri, Original version by (c) 1999 Tony Monroe")
    .arg(
      Arg::with_name("borg")
        .short("b")
        .long("borg")
        .help("borg eye"),
    )
    .arg(
      Arg::with_name("dead")
        .short("d")
        .long("dead")
        .help("dead eye and tongue"),
    )
    .arg(
      Arg::with_name("greedy")
        .short("g")
        .long("greedy")
        .help("greedy eye"),
    )
    .arg(
      Arg::with_name("paranoid")
        .short("p")
        .long("paranoid")
        .help("paranoid eye"),
    )
    .arg(
      Arg::with_name("stoned")
        .short("s")
        .long("stoned")
        .help("stoned eye and tongue"),
    )
    .arg(
      Arg::with_name("tired")
        .short("t")
        .long("tired")
        .help("tired eye"),
    )
    .arg(
      Arg::with_name("wired")
        .short("w")
        .long("wired")
        .help("wired eye"),
    )
    .arg(
      Arg::with_name("young")
        .short("y")
        .long("young")
        .help("young eye"),
    )
    .arg(
      Arg::with_name("tongue")
        .short("T")
        .long("tongue")
        .takes_value(true)
        .help("specify tongue"),
    )
    .arg(
      Arg::with_name("eyes")
        .short("e")
        .long("eyes")
        .takes_value(true)
        .help("specify eyes"),
    )
    .arg(
      Arg::with_name("file")
        .short("f")
        .long("file")
        .takes_value(true)
        .help("select which cow to use"),
    )
    .arg(
      Arg::with_name("wrapcols")
        .short("W")
        .long("wrapcols")
        .takes_value(true)
        .validator(validator_wrapcols)
        .help("column num to wrap line"),
    )
    .arg(
      Arg::with_name("what-L")
        .short("L")
        .help("unknown parameter in original version."),
    )
    .arg(
      Arg::with_name("what-N")
        .short("N")
        .help("unknown parameter in original version."),
    )
    .arg(
      Arg::with_name("list")
        .short("l")
        .long("list")
        .help("list cow files"),
    )
    .arg(Arg::with_name("msg").multiple(true).help("message"))
}

fn validator_wrapcols(s: String) -> Result<(), String> {
  let n = s
    .parse::<usize>()
    .expect("-W option takes integer larger than 0");
  if n <= 0 {
    Err(String::from("-W option takes integer larger than 0"))
  } else {
    Ok(())
  }
}
