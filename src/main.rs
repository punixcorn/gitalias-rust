/*
    gitalias-rust
    a rewrite of gitalias in rust
    author: punixcorn@2023
*/

#![allow(non_camel_case_types)]
#![allow(unused_mut)]
#![allow(unused)]
#![allow(non_snake_case)]

use clap::{command, Args};
// requires
use clap::{Parser, Subcommand};
use std::env;
use std::error::Error;
use std::fmt::*;
use std::format_args;
use std::fs;
use std::path::Display;
use std::process;
use std::process::*;
use std::str;
use std::string;
use std::stringify;
use std::vec::*;
use std::{clone, path};

/* Hold Global info */
#[derive(Debug)]
struct Globals {
    /// the message arg
    message: Vec<String>,
    /// the add files arg
    add: Vec<String>,
    /// creating online repo, the repo name
    reponame: String,
    /// creating online repo, the repo description
    repodes: String,
    /// creating online repo, the repo public or private
    mode: bool,
    /// the undo args
    undocommand: Vec<String>,
}

impl Globals {
    fn new() -> Self {
        Self {
            message: vec![String::from("-m")],
            add: vec![String::from("add")],
            reponame: String::from(""),
            repodes: String::from(""),
            mode: false,
            undocommand: vec![String::from("")],
        }
    }
}

// /// get a program stdio,stderr, status in the primatives
// #[derive(Debug)]
// struct ProgramOutput {
//     stdout: String,
//     stderr: String,
//     status: u8,
//     output: Output,
// }

// impl ProgramOutput {
//     fn new(cmd: &mut Command) -> Self {
//         Self {
//             stdout: getOutput(cmd, c),
//             stderr: (),
//             status: (),
//             output: (),
//         }
//     }
// }

/// print for more information with program_name
fn help(programName: &String) -> () {
    println!("for more information, try {} --help", programName);
}

/// print {s} and exit(1)
fn error(errorMessage: &String) -> () {
    println!("{}", errorMessage);
    exit(1);
}

/* use doesFileExist
fn findInDir(path: &String, find: &String) -> bool {
    let entries = fs::read_dir(path);
    let mut retVal: bool = false;
    for file in entries.unwrap() {
        let entry = file.unwrap();
        let filename_buf = entry.file_name();
        let filename = filename_buf.to_str().unwrap();

        if String::from(filename) == *find {
            retVal = true;
        }
    }
    retVal
}
*/

/// check if path_to_file exists [ returns: bool ]
fn doesFileExist<T>(filePath: &T) -> bool
where
    T: std::convert::AsRef<std::ffi::OsStr>,
{
    return path::Path::new(filePath).exists();
}

/// converts Output into String [ returns: String ]
fn convertOuputToString(out: Output) -> String {
    let string = String::from_utf8(out.stdout).unwrap();
    string.clone()
}

/// checks for untracked files in a repo
fn untrackedFilesExists() -> bool {
    let result = convertOuputToString(getOutput(&mut Command::new("/bin/git"), &[&"status"]));
    if result.find("Changes not staged for commit:") != None {
        true
    } else if result
        .find("nothing added to commit but untracked files present (use \"git add\" to track)")
        != None
    {
        true
    } else if result.find("Untracked files:") != None {
        true
    } else {
        false
    }
}
///checks if git is init'ed
fn isGitInit() -> bool {
    return doesFileExist(&"./.git");
}

/// creates an online repository based on g
fn createOnlineRepo(g: &mut Globals) -> () {
    let mut token: String =
        String::from(String::from_utf8(fs::read("/usr/githubToken").unwrap()).unwrap());
    if token == "" && (token.len() > 41 || token.len() < 41) {
        error(&String::from("gitHub token not found in /usr/githubToken"));
    }
    // remove newline
    token.pop();
    println!(
        "Online repository details :\n Name: {}\n
    Description: {}\nVisibility: {}\nDo you want to continue[y,N]: ",
        g.reponame, g.repodes, g.mode
    );

    let answer = std::io::stdin().lines().next().unwrap().unwrap();

    if answer == "Y" || answer == "y" {
        let modeStr = {
            match g.mode {
                true => "true",
                _ => "false",
            }
        };
        let runString = format!("curl -X POST -H \"Authorization: Bearer {}\" https://api.github.com/user/repos -d '{{\"name\":\"{}\",\"description\":\"{}\",\"homepage\":\"https://github.com\",\"private\":{}}}'",
        token, g.reponame,g.repodes,modeStr);

        // check if curl is downloaded
        let curl = doesFileExist(&String::from("/bin/curl"));
        if !curl {
            error(&String::from("Curl is needed to create online repository"));
        }
        // run bash and curl
        let returnString = convertOuputToString(getOutput(
            &mut Command::new("/bin/bash"),
            &[&String::from("-c"), &runString],
        ));

        // handle errors
        if returnString.find("Bad credentials") != None {
            error(&String::from(
                "Bad credentials : Please update your token, it may have expired",
            ));
        };
        if returnString.find("Could not resolve host: api.github.com") != None {
            error(&String::from(
                "Could not connect to api.github.com\ncheck internet connection",
            ));
        };
        println!("{} repository created succesfully", g.reponame);
    } else {
        println!("online repository creation cancelled");
    }
    ()
}

/// pass args c into Command cmd and run [ returns: Output ]
fn getOutput<T>(cmd: &mut process::Command, c: &[&T]) -> Output
where
    T: std::convert::AsRef<std::ffi::OsStr> + ?Sized,
{
    cmd.args(c).output().unwrap()
}

/// pass args into git and run and print the output [ returns () ]
fn runGit<T>(arr: &[&T]) -> ()
where
    T: std::convert::AsRef<std::ffi::OsStr> + ?Sized,
{
    print!(
        "{}",
        convertOuputToString(getOutput(&mut process::Command::new("/bin/git"), arr))
    );
}

#[derive(Parser)]
#[command(name = "gitalias")]
#[command(author = "punixcorn <cookedpotato663@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "a simple git alias", long_about = None)]
struct options {
    /// init a repository
    #[arg(short,long,action = clap::ArgAction::Count)]
    init: u8,

    /// commit added files
    #[arg(short, long,action = clap::ArgAction::Count)]
    commit: u8,

    /// add files to commit
    #[arg(short, long)]
    add: Option<Vec<String>>,

    /// add a commit message
    #[arg(short, long)]
    message: Option<Vec<String>>,

    /// create a branch
    #[arg(short, long)]
    branch: Option<String>,

    /// switch branches
    #[arg(short, long)]
    switch: Option<String>,

    /// delete a branch
    #[arg(short, long)]
    delete: Option<String>,

    /// Merge a [ branch ] to current working branch
    #[arg(short = 'M', long)]
    Merge: Option<String>,

    /// Pull from online repository
    #[arg(short = 'P', long)]
    Pull: Option<String>,

    /// push into online repository
    #[arg(short, long)]
    push: Option<String>,

    /// Clone [ username/repository ] or [ URL ]
    #[arg(short = 'C', long)]
    Clone: Option<String>,

    /// Protcol to use when Cloning [ ssh ] or [ https ]
    #[arg(short='R', long, default_value_t = String::from("https"))]
    Request: String,

    /// add a remote repository to local repository : [ username/repository ]
    #[arg(short, long)]
    origin: Option<String>,

    /// name for online repository [creating online repo]
    #[arg(short, long, group = "_repo", requires = "_Des")]
    repo: Option<String>,

    /// Description for online repository [creating online repo]
    #[arg(short = 'D', long, requires = "_repo", group = "_Des")]
    Description: Option<String>,

    /// if online repository should be [ private ] or [ public ]  [creating online repo]
    #[arg(
        short = 't',
        long,
        requires = "_repo",
        requires = "_Des",
        default_value_t = String::from("private")
    )]
    Type: String,

    /// reset back a commit [ hard/soft/mixed ] && [no of commits back]
    #[arg(short, long)]
    undo: Option<Vec<String>>,

    /// download a specific folder or file [ url ]
    #[arg(short = 'G', long)]
    Grab: Option<String>,
}

fn main() {
    let mut G: Globals = Globals::new();
    let mut cmdArgs: Vec<String> = env::args().collect();

    if env::args().count() <= 1 {
        println!("{} a git alias", cmdArgs[0]);
        help(&cmdArgs[0]);
        exit(1);
    } else {
        let args = options::parse();

        match args.init {
            0 => {}
            _ => {
                if (!isGitInit()) {
                    runGit(&[&"init"]);
                } else {
                    println!("Initalized repository already found")
                }
            }
        }

        match args.add {
            Some(pass) => {
                for i in pass {
                    G.add.push(i);
                }
                // getOutput(
                //     &mut Command::new("/bin/git"),,
                // );
            }
            None => {}
        }

        match args.commit {
            0 => {}
            _ => {}
        }

        match args.message {
            Some(pass) => for i in pass {},
            None => {}
        }

        match args.switch {
            Some(pass) => {
                runGit(&[&String::from("switch"), &pass]);
            }
            None => {}
        }
        match args.origin {
            Some(pass) => {
                let mut gitHubUsername: String = format!("git@github.com:{}", pass);

                /// checking if it is a valid string
                if (gitHubUsername.find("/") == None) {
                    error(&String::from(
                        "ERR: Origin Argument should be 'username/repository'",
                    ));
                }

                if (!isGitInit()) {
                    error(&String::from(
                        "ERR: no local repository found here cannot push to origin",
                    ));
                }

                println!("========== Adding Origin : {} ==========", gitHubUsername);
                let addOrigin = getOutput(
                    &mut Command::new("/bin//git"),
                    (&["remote", "add", "origin", &gitHubUsername]),
                );
                if !addOrigin.status.success() {
                    error(&String::from_utf8(addOrigin.stderr).unwrap());
                } else {
                    println!("origin added");
                };

                let changeBranch =
                    getOutput(&mut Command::new("/bin/git"), &["branch", "-M", "main"]);
                if !changeBranch.status.success() {
                    error(&String::from_utf8(changeBranch.stderr).unwrap());
                } else {
                    println!("branch switched to main");
                }

                let pushBranch = getOutput(
                    &mut Command::new("/bin/git"),
                    (&["push", "-u", "origin", "main"]),
                );
                if !pushBranch.status.success() {
                    error(&String::from_utf8(pushBranch.stderr).unwrap());
                } else {
                    println!("pushed to origin main");
                }
                println!("========== Done ==========");
            }
            None => {}
        }

        exit(0)
    }
}
