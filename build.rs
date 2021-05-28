use clap::Shell;

include!("src/cli.rs");

fn main() {
  let mut app = build_cli();
  app.gen_completions("rusty-cowsay", Shell::Bash, "./");
}
