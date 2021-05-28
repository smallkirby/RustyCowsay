// Credit of the original(Perl) version is below:
// (c) 19999 Tony Monroe
// This version of this code is under the same license. See LICENSE for the detail.

/* encoding: utf-8 */
/* coding: 2spaces */

use regex::Regex;
use std::io::Read;
use std::path;
use textwrap;

use clap::{App, Arg};

#[derive(Default, Debug)]
pub struct Opts {
  help: bool,
  version: bool,
  list: bool,
  mora: bool,
  think: bool,
  which: Option<String>,
  borg: bool,
  dead: bool,
  greedy: bool,
  paranoid: bool,
  stoned: bool,
  tired: bool,
  wired: bool,
  young: bool,
  tongue: Option<String>,
  eyes: Option<String>,
  wrap_cols: usize,
}

fn main() {
  let mut opts = Opts {
    ..Default::default()
  };
  let msg = parse_opts(&mut opts);

  if opts.list {
    match list_cowfiles() {
      Ok(_) => return,
      Err(msg) => println!("Err: {}", msg),
    };
  }
  match msg {
    Some(msg) => match display(&msg, &opts) {
      Ok(()) => return,
      Err(msg) => println!("Err: {}", msg),
    },
    None => {
      let mut buffer = String::new();
      let mut stdin = std::io::stdin();
      match stdin.read_to_string(&mut buffer) {
        Ok(_) => match display(&mut buffer, &opts) {
          Ok(()) => return,
          Err(msg) => println!("Err: {}", msg),
        },
        Err(_) => println!("Use --help option to show usage"),
      }
    }
  };
}

pub fn display(msg: &String, opts: &Opts) -> Result<(), String> {
  let mut msgs: Vec<String> = vec![];
  let _ = msg
    .split("\n")
    .map(|m| {
      let mut s = String::from(m);
      while !s.is_empty() {
        let (chunk, rest) = s.split_at(std::cmp::min(From::from(opts.wrap_cols), s.len()));
        msgs.push(String::from(chunk));
        s = String::from(rest);
      }
    })
    .collect::<Vec<_>>();
  let msgnum = msgs.len();
  let maxlen: usize = msgs.iter().map(|s| s.len()).max().unwrap();

  let lines = if msgnum < 2 {
    format!("< {} >\n", msg)
  } else {
    [
      format!(
        "/ {sentence}{spaceright} \\\n",
        sentence = msgs[0],
        spaceright = " ".repeat(maxlen - msgs[0].len())
      ),
      msgs[1..msgnum - 1]
        .iter()
        .map(|x| {
          format!(
            "| {sentence}{spaceright} |\n",
            sentence = x,
            spaceright = " ".repeat(maxlen - x.len()),
          )
        })
        .collect(),
      format!(
        "\\ {sentence}{spaceright} /\n",
        sentence = msgs[msgnum - 1],
        spaceright = " ".repeat(maxlen - msgs[msgnum - 1].len())
      ),
    ]
    .join("")
  };
  println!(
    " {}\n{} {}",
    "_".repeat(maxlen + 1),
    lines,
    "-".repeat(maxlen + 1)
  );

  // print cow
  print_cow(&opts)
}

pub fn print_cow(opts: &Opts) -> Result<(), String> {
  // construct face
  let eyes = if let Some(t) = &opts.eyes {
    match t.len() {
      1 => String::from(format!("{}{}", t, t)),
      2 => String::from(t),
      _ => String::from(&t[0..=2]),
    }
  } else if opts.borg {
    String::from("==")
  } else if opts.dead {
    String::from("xx")
  } else if opts.greedy {
    String::from("$$")
  } else if opts.paranoid {
    String::from("@@")
  } else if opts.stoned {
    String::from("**")
  } else if opts.tired {
    String::from("--")
  } else if opts.wired {
    String::from("OO")
  } else if opts.young {
    String::from("..")
  } else {
    String::from("oo")
  };
  let tongue = if let Some(t) = &opts.tongue {
    match t.len() {
      1 => String::from(t),
      _ => String::from(&t[0..=0]),
    }
  } else if opts.dead {
    String::from("U")
  } else if opts.stoned {
    String::from("U")
  } else {
    String::from(" ")
  };
  let thoughts = if opts.think { "o" } else { "\\" };

  // print cow
  let cwd = std::env::current_dir().unwrap();
  let mut cowpath = if let Ok(ref p) = std::env::var("COWPATH") {
    path::PathBuf::from(p)
  } else {
    [cwd.as_os_str().to_str().unwrap(), "cows"].iter().collect()
  };

  if opts.mora {
    cowpath.push("mora.cow");
  } else if let Some(file) = &opts.which {
    let cowfile = &Regex::new(r"^(.+)(\.cow)?$")
      .unwrap()
      .captures(file)
      .unwrap()[1];
    cowpath.push(format!("{}.cow", cowfile));
  } else {
    cowpath.push("bong.cow");
  }
  let cow = if let Ok(cow) = std::fs::read_to_string(cowpath) {
    cow
  } else {
    return Err(String::from("Cow doesn't here..."));
  };
  println!(
    "{}",
    cow.split("\n").collect::<Vec<&str>>()[1..]
      .join("\n")
      .replace("$thoughts", thoughts)
      .replace("$eyes", &eyes)
      .replace("$tongue", &tongue)
  );
  Ok(())
}

pub fn parse_opts(opts: &mut Opts) -> Option<String> {
  let app = App::new("rusty cow{{say,think}}")
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
      Arg::with_name("list")
        .short("l")
        .long("list")
        .help("list cow files"),
    )
    .arg(Arg::with_name("msg").multiple(true).help("message"));
  let matches = app.get_matches();

  if matches.is_present("help") {
    opts.help = true;
  }
  if matches.is_present("version") {
    opts.version = true;
  }
  if matches.is_present("list") {
    opts.list = true;
  }
  if matches.is_present("borg") {
    opts.borg = true;
  }
  if matches.is_present("dead") {
    opts.dead = true;
  }
  if matches.is_present("greedy") {
    opts.greedy = true;
  }
  if matches.is_present("paranoid") {
    opts.paranoid = true;
  }
  if matches.is_present("greedy") {
    opts.greedy = true;
  }
  if matches.is_present("tired") {
    opts.tired = true;
  }
  if matches.is_present("wired") {
    opts.wired = true;
  }
  if matches.is_present("young") {
    opts.young = true;
  }
  if let Some(tongue) = matches.value_of("tongue") {
    opts.tongue = Some(String::from(tongue));
  };
  if let Some(eyes) = matches.value_of("eyes") {
    opts.eyes = Some(String::from(eyes));
  };
  if let Some(file) = matches.value_of("file") {
    opts.which = Some(String::from(file));
  };
  if let Some(w) = matches.value_of("wrapcols") {
    opts.wrap_cols = w.parse::<usize>().unwrap()
  } else {
    opts.wrap_cols = 80
  };
  let args: Vec<String> = std::env::args().collect();
  if path::Path::new(&args[0])
    .file_name()
    .unwrap()
    .to_str()
    .unwrap()
    == "morasay"
  {
    opts.mora = true;
  } else if path::Path::new(&args[0])
    .file_name()
    .unwrap()
    .to_str()
    .unwrap()
    == "cowthink"
  {
    opts.think = true;
  }
  if let Some(msgs) = matches.values_of("msg") {
    //Some(msgs.join())
    Some(msgs.collect::<Vec<&str>>().join(" "))
  } else {
    None
  }
}

pub fn list_cowfiles() -> Result<(), String> {
  let cwd = std::env::current_dir().unwrap();
  let cowpath = if let Ok(ref p) = std::env::var("COWPATH") {
    path::PathBuf::from(p)
  } else {
    [cwd.as_os_str().to_str().unwrap(), "cows"].iter().collect()
  };

  println!("Cow files in {}:", cowpath.to_str().unwrap());
  let cowpath_dir = if let Ok(d) = std::fs::read_dir(cowpath) {
    d
  } else {
    return Err(String::from("Dir doesn't exist."));
  };
  let cowregex = Regex::new(r"(.+)\.cow").unwrap();
  let mut cows: Vec<String> = vec![];
  for path in cowpath_dir {
    match path {
      Ok(p) => {
        if let Some(result) = cowregex.captures(&p.file_name().to_string_lossy()) {
          cows.push(String::from(&result[1]));
        }
      }
      Err(_) => println!("(skipping one file)"),
    }
  }
  println!("{}", textwrap::fill(&cows.to_owned().join(" "), 80));

  return Ok(());
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

#[cfg(test)]
mod tests {
  #[test]
  fn default_oneline_cowsay() {
    let opts = super::Opts {
      help: false,
      version: false,
      list: false,
      mora: false,
      ..Default::default()
    };
    let msg = String::from("waiwai");
    assert_eq!((), super::display(&msg, &opts).unwrap());
  }
}
