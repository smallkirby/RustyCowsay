// Credit of the original(Perl) version is below:
// (c) 19999 Tony Monroe
// This version of this code is under the same license. See LICENSE for the detail.

/* encoding: utf-8 */
/* coding: 2spaces */

use regex::Regex;
use std::io::{self, Read};
use std::path;
use textwrap;

const VERSION: &str = "0.0.1";

use clap::{App, Arg};

#[derive(Default, Debug)]
pub struct Opts {
  help: bool,
  version: bool,
  list: bool,
  mora: bool,
  think: bool,
}

fn main() {
  let mut opts = Opts {
    ..Default::default()
  };
  let msg = parse_opts(&mut opts);

  if opts.version {
    show_version_credit();
    return;
  }
  if opts.help {
    show_help();
    return;
  }
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
        Err(_) => show_help(),
      }
    }
  };
}

pub fn display(msg: &String, opts: &Opts) -> Result<(), String> {
  let mut msgnum = msg.split("\n").collect::<Vec<&str>>().len();
  let maxlen: usize = msg.split("\n").map(|s| s.len()).max().unwrap();
  let lines = if msgnum < 2 {
    format!("< {} >\n", msg)
  } else {
    let mut msgs = msg.split("\n").collect::<Vec<&str>>();
    if msgs[msgnum - 1].len() == 0 {
      msgs = msgs[..msgnum].into();
      msgnum -= 1;
    }
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
  // XXX not imp
  let eyes = "oo";
  let tongue = "U";
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
  } else {
    cowpath.push("bong.cow"); // XXX
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
      .replace("$eyes", eyes)
      .replace("$tongue", tongue)
  );
  Ok(())
}

pub fn parse_opts(opts: &mut Opts) -> Option<String> {
  let app = App::new("rusty-cowsay")
    .version(VERSION)
    .arg(
      Arg::with_name("help")
        .short("h")
        .long("help")
        .help("show help"),
    )
    .arg(
      Arg::with_name("version")
        .short("v")
        .long("version")
        .help("show version info"),
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

pub fn show_version_credit() {
  println!("cow{{say,think}} version {}, (c) 2021 Nirugiri", VERSION);
  println!("Original version by (c) 1999 Tony Monroe");
}

pub fn show_help() {
  show_version_credit();
  println!("Usage: cowsay [-h, --help] [-v --version] [message]");
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
