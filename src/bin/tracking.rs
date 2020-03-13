extern crate clap;

use clap::Clap;
use chrono::prelude::*;

use tracking::*;

#[derive(Clap)]
struct AppOpts {
    #[clap(subcommand)]
    sub_command: SubCommand
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "start")]
    StartCmd(StartOpts),
    #[clap(name = "stop")]
    StopCmd(StopOpts),
}

#[derive(Clap)]
struct StartOpts {
    task: String
}

#[derive(Clap)]
struct StopOpts {
    task: String
}

fn main() {
    let opts = AppOpts::parse();

    match opts.sub_command {
        SubCommand::StartCmd(opts) => {
            let mut task = Task {
                name: opts.task,
                events: Vec::new(),
            };

            let event = TimeEvent::Start(Utc::now());

            println!("Starting task \"{}\" @ {}", task.name, event.get_time());

            task.events.push(event);
        },

        SubCommand::StopCmd(opts) => {
            let mut task = Task {
                name: opts.task,
                events: Vec::new(),
            };

            let event = TimeEvent::Stop(Utc::now());

            println!("Stopping task \"{}\" @ {}", task.name, event.get_time());

            task.events.push(event);
        }
    }
}