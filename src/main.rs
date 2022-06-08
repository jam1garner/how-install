use std::collections::HashMap;

use clap::{ArgEnum, Parser};
use displaydoc::Display;
use nix::unistd::geteuid;
use owo_colors::OwoColorize;
use reqwest::Error;
use scraper::{Html, Selector};

use how_install_tealdeer as tealdeer;

#[derive(Parser)]
#[clap(
    about = "A CLI for helping find how to install a given command",
    long_about = "A CLI for helping find how to install a given command\n\n\
Credit to:\
\n   - https://tldr.sh for descriptions\
\n   - https://dbrgn.github.io/tealdeer/ for tldr console output\
\n   - https://command-not-found.com/ for command install information"
)]
struct Args {
    /// Command to lookup how to install
    cmd: String,

    /// Run install command
    #[clap(long, short)]
    install: bool,

    /// Automatically run install command without prompting
    #[clap(short)]
    yes: bool,

    /// Don't output TLDR info about the given command
    #[clap(long)]
    no_tldr: bool,

    /// OS to install for
    #[clap(long, arg_enum, ignore_case = true)]
    distro: Option<LinuxDistro>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Display)]
enum LinuxDistro {
    /// Debian
    Debian,

    /// Ubuntu
    Ubuntu,

    /// Alpine
    Alpine,

    /// Arch
    Arch,

    /// Kali
    Kali,

    /// CentOS
    Centos,

    /// Fedora
    Fedora,

    /// Raspbian
    Raspbian,

    /// Docker Container
    Docker,
}

fn error(msg: impl AsRef<str>) -> ! {
    let msg = msg.as_ref();

    eprintln!("{msg}");

    std::process::exit(1)
}

fn confirm_install(command: &str) -> bool {
    atty::isnt(atty::Stream::Stdout)
        || dialoguer::Confirm::new()
            .with_prompt(format!("Install {command} using the above command?"))
            .default(true)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let command = &args.cmd;
    let url = format!("https://command-not-found.com/{}", command);

    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse(".command-install:not(.d-none)").unwrap();

    let dt_selector = Selector::parse("dt").unwrap();
    let dd_selector = Selector::parse("dd").unwrap();

    let os = sys_info::linux_os_release().expect("Failed to get Linux OS release info");

    let command: HashMap<_, _> = document
        .select(&selector)
        .flat_map(|element| {
            let name = element
                .select(&dt_selector)
                .next()
                .unwrap()
                .text()
                .last()
                .unwrap()
                .trim()
                .to_string();

            let data_os_name = if let Some(data_os_name) = element.value().attr("data-os") {
                data_os_name.to_owned()
            } else {
                String::from("")
            };

            const PREFIX: &str = "install-";

            let id = if let Some(id) = element
                .value()
                .classes()
                .find(|class| class.starts_with(PREFIX))
            {
                id[PREFIX.len()..].to_owned()
            } else {
                return vec![];
            };

            let command = element
                .select(&dd_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            vec![
                (name, command.clone()),
                (data_os_name, command.clone()),
                (id, command),
            ]
        })
        .collect();

    let command = if let Some(distro) = args.distro {
        match command.get(&distro.to_string()) {
            Some(command) => command,
            None => error(format!("{cmd} not found for {distro}", cmd = args.cmd)),
        }
    } else {
        command
            .get(os.name())
            .or_else(|| command.get(os.pretty_name()))
            .or_else(|| command.get(os.id()))
            .unwrap_or_else(|| {
                error(format!(
                    "Failed to find install command for {:?} on OS {:?}",
                    args.cmd,
                    os.pretty_name(),
                ))
            })
    };

    let maybe_sudo = if geteuid().is_root() { "" } else { "sudo " };

    if !args.no_tldr && tealdeer::list().await.contains(&args.cmd) {
        eprint!("{}", "TLDR".bold());
        tealdeer::main(args.cmd.clone()).await;
    }

    eprint!("{}", "INSTALL\n  ".bold());
    println!("{maybe_sudo}{command}");

    if args.yes || (args.install && confirm_install(&args.cmd)) {
        let status = std::process::Command::new("bash")
            .args(&["-c", command])
            .status()
            .unwrap();

        std::process::exit(status.code().unwrap());
    }

    Ok(())
}
