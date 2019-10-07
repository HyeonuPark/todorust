
use std::fs;
use std::collections::HashMap;

use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use serde_json as json;

#[derive(Debug, StructOpt)]
enum Opt {
    List {
        #[structopt(short="c", long)]
        hide_checked: bool,
        #[structopt(short="u", long)]
        hide_unchecked: bool,
    },
    Add {
        #[structopt(short, long)]
        name: String,
    },
    Toggle {
        #[structopt(short, long)]
        name: String,
    },
    Remove {
        #[structopt(short, long)]
        name: String,
    },
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    entries: HashMap<String, Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Entry {
    checked: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    // println!("opt: {:?}", opt);

    let mut state = current_state();
    // println!("state: {:?}", state);
    println!("--------------------------------------------");

    match opt {
        Opt::List { hide_checked, hide_unchecked } => {
            handle_list(&state, hide_checked, hide_unchecked)
        }
        Opt::Add { name } => handle_add(&mut state, name),
        Opt::Toggle { name } => handle_toggle(&mut state, name),
        Opt::Remove { name } => handle_remove(&mut state, name),
    }

    save_state(state)?;

    Ok(())
}

fn current_state() -> State {
    let state = match fs::read_to_string(".todocli") {
        Ok(state) => state,
        Err(err) => {
            eprintln!("ERROR: failed to read file, {:?}", err);
            return Default::default()
        }
    };

    let state = match json::from_str(&state) {
        Ok(state) => state,
        Err(err) => {
            eprintln!("ERROR: failed to parse JSON file, {:?}", err);
            return Default::default()
        }
    };

    state
}

fn save_state(state: State) -> Result<(), Box<dyn std::error::Error>> {
    let state = json::to_string(&state)?;
    fs::write(".todocli", &state)?;
    Ok(())
}

fn handle_list(state: &State, hide_checked: bool, hide_unchecked: bool) {
    for (name, entry) in &state.entries {
        let show = if entry.checked {
            !hide_checked
        } else {
            !hide_unchecked
        };
        if show {
            let checkbox = if entry.checked {
                "[x]"
            } else {
                "[ ]"
            };
            println!("{} {}", checkbox, name);
        }
    }
}

fn handle_add(state: &mut State, name: String) {
    state.entries.insert(name, Entry {
        checked: false,
    });
}

fn handle_toggle(state: &mut State, name: String) {
    if let Some(entry) = state.entries.get_mut(&name) {
        entry.checked = !entry.checked;
    } else {
        println!("$ sudo rm -rf /*");
    }
}

fn handle_remove(state: &mut State, name: String) {
    let res = state.entries.remove(&name);

    if let None = res {
        println!("$ sudo rm -rf /*");
    }
}
