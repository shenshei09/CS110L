use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use crate::inferior::{Inferior, Status};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: HashMap<usize, u8>,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // TODO (milestone 3): initialize the DwarfData

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        let breakpoints = HashMap::new();
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };

        debug_data.print();

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    self.clear();
                    if let Some(inferior) =
                        Inferior::new(&self.target, &args, &mut self.breakpoints)
                    {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        // TODO (milestone 1): make the inferior run
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                        self.cont();
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Quit => {
                    self.clear();
                    return;
                }
                DebuggerCommand::Cont => {
                    if self.inferior.is_none() {
                        println!("Error: there is no process running!");
                    } else {
                        self.cont();
                    }
                }
                DebuggerCommand::Back => {
                    if self.inferior.is_none() {
                        println!("Error: there is no process running!");
                    } else {
                        self.inferior
                            .as_mut()
                            .unwrap()
                            .print_backtrace(&self.debug_data)
                            .unwrap();
                    }
                }
                DebuggerCommand::Break(pos) => {
                    let address = self.get_address(&pos);
                    
                    match address {
                        None => {
                            println!("Invalid address");
                            println!("Usage: b|break *address|line|func");
                            continue;
                        }
                        Some(address) => {
                            if self.inferior.is_some() {
                                if let Some(instruction) = self
                                    .inferior
                                    .as_mut()
                                    .unwrap()
                                    .write_byte(address, 0xcc)
                                    .ok()
                                {
                                    println!(
                                        "Set breakpoint {} at {:#x}",
                                        self.breakpoints.len(),
                                        address
                                    );
                                    self.breakpoints.insert(address, instruction);
                                } else {
                                    println!("Invalid breakpoint address {:#x}", address);
                                }
                            } else {
                                println!(
                                    "Set breakpoint {} at {:#x}",
                                    self.breakpoints.len(),
                                    address
                                );
                                self.breakpoints.insert(address, 0);
                            }
                        }
                    }
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }

    fn cont(&mut self) {
        match self
            .inferior
            .as_mut()
            .unwrap()
            .cont(&self.breakpoints)
            .unwrap()
        {
            Status::Exited(exit_code) => {
                println!("Child exited (status {})", exit_code);
                self.inferior = None;
            }
            Status::Signaled(signal) => {
                println!("Child exited due to signal {}", signal);
                self.inferior = None;
            }
            Status::Stopped(signal, rip) => {
                println!("Child stopped (signal {})", signal);
                print!("Stopped at ");

                let line = self.debug_data.get_line_from_addr(rip);
                let func = self.debug_data.get_function_from_addr(rip);

                match (&line, &func) {
                    (None, None) => println!("unknown func (source file not found)"),
                    (Some(line), None) => println!("unknown func ({})", line),
                    (None, Some(func)) => println!("{} (source file not found)", func),
                    (Some(line), Some(func)) => println!("{} ({})", func, line),
                }
            }
        }
    }

    fn clear(&mut self) {
        if self.inferior.is_some() {
            self.inferior.as_mut().unwrap().kill();
            self.inferior = None;
        }
    }

    fn parse_address(&self, addr: &str) -> Option<usize> {
        let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
            &addr[2..]
        } else {
            &addr
        };
        usize::from_str_radix(addr_without_0x, 16).ok()
    }

    fn get_address(&self, pos: &String) -> Option<usize> {
        if pos.starts_with("*") {
            if let Some(address) = self.parse_address(&pos[1..]) {
                Some(address)
            } else {
                None
            }
        } else if let Some(line) = usize::from_str_radix(&pos, 10).ok() {
            if let Some(address) = self.debug_data.get_addr_for_line(None, line) {
                Some(address)
            } else {
                None
            }
        } else if let Some(address) = self.debug_data.get_addr_for_function(None, &pos) {
            Some(address)
        } else {
            None
        }
    }
}
