use std::io;
use std::io::Write;
use std::error::Error;
use rust_capstone::vault::*;

enum Action {
    NewVault,
    OpenVault,
}

fn initialize_vault(password: &str) -> DummyVault {
    let mut vault = DummyVault::default();
    vault.initialize(password);
    vault
}
fn open_vault(password: &str) -> Result<DummyVault, Box<dyn Error>> {
    let vault = DummyVault::default();
    vault.get(password)
}

fn prompt_master_password() -> Result<String, Box<dyn Error>> {
    print!("Enter master password: ");
    io::stdout().flush()?;
    let mut master_password = String::new();
    io::stdin().read_line(&mut master_password)?;
    let master_password = master_password.trim();
    Ok(master_password.to_owned())
}

fn prompt_for_action() -> Result<Action, Box<dyn Error>> {
    println!("Do you want to (n)ew vault or (o)pen existing one?");
    let mut action = String::new();
    io::stdin().read_line(&mut action)?;
    match action.trim() {
        "n" => Ok(Action::NewVault),
        "o" => Ok(Action::OpenVault),
        _ => Err("Invalid action".into()),
    }
}

fn display_records(vault: &DummyVault) {
    for (i, record) in vault.data.records.iter().enumerate() {
        println!("{}: {}", i + 1, record.title);
    }
}

fn add_record(vault: &mut DummyVault) -> Result<(), Box<dyn Error>> {
    print!("Enter title: ");
    io::stdout().flush()?;
    let mut title = String::new();
    io::stdin().read_line(&mut title)?;
    
    print!("Enter password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    
    vault.data.records.push(Record { title: title.trim().to_string(), password: password.trim().to_string() });
    Ok(())
}

fn save_vault(vault: &DummyVault, master_password: &str) -> Result<(), Box<dyn Error>> {
    vault.push(&vault.data, master_password)
}

pub fn main() {
    match prompt_master_password() {
        Ok(master_password) => {
            match prompt_for_action() {
                Ok(action) => {
                    let mut vault = match action {
                        Action::NewVault => initialize_vault(&master_password),
                        Action::OpenVault => match open_vault(&master_password) {
                            Ok(v) => v,
                            Err(e) => {
                                println!("Error opening vault: {}", e);
                                return;
                            }
                        },
                    };

                    loop {
                        println!("Choose action: (l)ist, (a)dd, (s)ave and exit");
                        let mut choice = String::new();
                        io::stdin().read_line(&mut choice).unwrap();

                        match choice.trim() {
                            "l" => display_records(&vault),
                            "a" => {
                                if let Err(e) = add_record(&mut vault) {
                                    println!("Error adding record: {}", e);
                                }
                            },
                            "s" => {
                                if let Err(e) = save_vault(&vault, &master_password) {
                                    println!("Error saving vault: {}", e);
                                }
                                break;
                            },
                            _ => println!("Invalid option."),
                        }
                    }
                },
                Err(e) => println!("Error: {}", e),
            }
        },
        Err(e) => println!("Failed to read password: {}", e),
    }
}