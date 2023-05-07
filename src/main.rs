use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{arg, command, Arg, Command};
use serde::{Deserialize, Serialize};

trait ToPath {
    fn path(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
struct Dog {
    name: String,
    owner: String,
    age: u8,
}

impl Dog {
    fn new(name: String, owner: String, age: u8) -> Self {
        Self { name, owner, age }
    }
}
impl ToPath for Dog {
    fn path(&self) -> String {
        format!("animals/dogs/{}.json", &self.name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Cat {
    name: String,
    owner: String,
    age: u8,
}

impl Cat {
    fn new(name: String, owner: String, age: u8) -> Self {
        Self { name, owner, age }
    }
}
impl ToPath for Cat {
    fn path(&self) -> String {
        format!("animals/cats/{}.json", &self.name)
    }
}

fn load_dog(name: &str) -> Result<Dog> {
    let path = format!("animals/dogs/{}.json", name);
    let json_str = fs::read_to_string(path)?;
    let dog: Dog = serde_json::from_str(&json_str)?;
    Ok(dog)
}

fn load_cat(name: &str) -> Result<Cat> {
    let path = format!("animals/cats/{}.json", name);
    let json_str = fs::read_to_string(path)?;
    let cat: Cat = serde_json::from_str(&json_str)?;
    Ok(cat)
}

fn list_dogs() -> Result<Vec<String>> {
    let mut dogs = Vec::new();
    for entry in fs::read_dir("animals/dogs")? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        dogs.push(name);
    }
    Ok(dogs)
}

fn list_cats() -> Result<Vec<String>> {
    let mut cats = Vec::new();
    for entry in fs::read_dir("animals/cats")? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        cats.push(name);
    }
    Ok(cats)
}

fn save<T: Serialize + ToPath>(animal: &T) -> Result<()> {
    let json_str = serde_json::to_string(&animal)?;
    fs::write(animal.path(), json_str)?;
    Ok(())
}

fn main() -> Result<()> {
    fs::create_dir_all(Path::new("animals/dogs"))?;
    fs::create_dir_all(Path::new("animals/cats"))?;
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("dog")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("list"))
                .subcommand(
                    Command::new("show")
                        .arg_required_else_help(true)
                        .arg(Arg::new("name").required(true)),
                )
                .subcommand(
                    Command::new("new")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("name")
                                .short('n')
                                .long("name")
                                .help("Name of the dog")
                                .required(true),
                        )
                        .arg(
                            Arg::new("owner")
                                .short('o')
                                .long("owner")
                                .help("Name of the owner")
                                .required(true),
                        )
                        .arg(
                            Arg::new("age")
                                .short('a')
                                .long("age")
                                .help("Age of the dog")
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("cat")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("list"))
                .subcommand(
                    Command::new("show")
                        .arg_required_else_help(true)
                        .arg(Arg::new("name").required(true)),
                )
                .subcommand(
                    Command::new("new")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("name")
                                .short('n')
                                .long("name")
                                .help("Name of the dog")
                                .required(true),
                        )
                        .arg(
                            Arg::new("owner")
                                .short('o')
                                .long("owner")
                                .help("Name of the owner")
                                .required(true),
                        )
                        .arg(
                            Arg::new("age")
                                .short('a')
                                .long("age")
                                .help("Age of the dog")
                                .required(true),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("dog", sub_matches)) => {
            match sub_matches.subcommand_matches("list") {
                Some(_) => {
                    let dog_names = list_dogs()?;
                    if dog_names.is_empty() {
                        println!("No dogs found");
                    }
                    for dog in dog_names {
                        println!("{}", dog);
                    }
                }
                None => {}
            }
            match sub_matches.subcommand_matches("new") {
                Some(new_matches) => {
                    let name = new_matches.get_one::<String>("name").unwrap();
                    let owner = new_matches.get_one::<String>("owner").unwrap();
                    let age = new_matches.get_one::<String>("age").unwrap();
                    let dog = Dog::new(name.to_owned(), owner.to_owned(), age.parse::<u8>()?);
                    save(&dog)?;
                }
                None => {}
            }
            match sub_matches.subcommand_matches("show") {
                Some(show_matches) => {
                    let name = show_matches.get_one::<String>("name").unwrap();
                    let dog = load_dog(name)?;
                    println!("{:?}", dog);
                }
                None => {}
            }
        }
        Some(("cat", sub_matches)) => {
            match sub_matches.subcommand_matches("list") {
                Some(_) => {
                    let cat_names = list_cats()?;
                    if cat_names.is_empty() {
                        println!("No cats found");
                    }
                    for cat in cat_names {
                        println!("{}", cat);
                    }
                }
                None => {}
            }
            match sub_matches.subcommand_matches("new") {
                Some(new_matches) => {
                    let name = new_matches.get_one::<String>("name").unwrap();
                    let owner = new_matches.get_one::<String>("owner").unwrap();
                    let age = new_matches.get_one::<String>("age").unwrap();
                    let cat = Cat::new(name.to_owned(), owner.to_owned(), age.parse::<u8>()?);
                    save(&cat)?;
                }
                None => {}
            }
            match sub_matches.subcommand_matches("show") {
                Some(show_matches) => {
                    let name = show_matches.get_one::<String>("name").unwrap();
                    let cat = load_cat(name)?;
                    println!("{:?}", cat);
                }
                None => {}
            }
        }
        _ => unreachable!("Exhaused list of subcommands and subcommand_required is true"),
    }

    Ok(())
}

// only if dog subcommand has a name argument
// if let Some(name) = sub_matches.get_one::<String>("NAME") {
//     println!("dog was used, his name is: {}", name)
// }
