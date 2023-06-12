extern crate clap;
mod repl;

use repl::{REPLHelper, get_config};
use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::{App, crate_version};

fn main() -> rustyline::Result<()> {
    env_logger::init();

    let _matches = App::new("Rust-SQLite")
                        .version("0.0.1")
                        .author("Mehul")
                        .about("Light version of SQLite developed using Rust")
                        .get_matches();

    let config=get_config();

    let helper = REPLHelper::new();

    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    if repl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut count = 1;
    loop{
        if count == 1 {
            println!("{}{}{}{}{}",
        format!("Rust-SQLite - {}\n", crate_version!()),
        "Enter .exit to quit.\n",
        "Enter .help for usage hints.\n",
        "Connected to a transient in-memory database.\n",
        "Use '.open FILENAME' to reopen on a persistent database.");
        //TODO: Get info about application name and version dynamically.
        }

        let p = format!("rust-sqlite | {}>", count);
        repl.helper_mut()
            .expect("No helper found")
            .colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
        let readline = repl.readline(&p);

        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                if command.eq(".exit") {
                    break;
                }else{
                    println!("Error: unknown command or invalid arguments : '{}'. Enter '.help'", &command);
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
        count += 1;
    }

    repl.save_history("history.txt").unwrap();

    Ok(())
}
