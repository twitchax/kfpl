#![warn(rust_2018_idioms, clippy::all)]
#![feature(trait_alias)]

mod helpers;
mod services;

use simple_logger::SimpleLogger;
use anyhow::{Result, Error};
use log::{
    warn, 
    LevelFilter
};
use clap::{Arg, App, SubCommand, ArgMatches};
use yansi::Paint;

use crate::services::{
    model::{EnsurableEntity, RemovableEntity},
    docker::Docker,
    k3d::K3d,
    kubectl::Kubectl,
    pip3::Pip3,
    kfp::Kfp,
    kfp_service::KfpService,
    port_forward::PortForward,
    k3d_service::K3dService,
    curl::Curl,
    git::Git
};

// TODO:
//   * Add git service.
//   * Allow for "auto yes-ing" the dependencies with `-y`.

#[tokio::main]
async fn main() -> Result<()> {

    // Set up logging.
    SimpleLogger::new().with_level(LevelFilter::Info);

    let args = App::new("kfpl")
        .version("1.0")
        .author("Aaron Roney")
        .about("Automates running KubeFlow Pipelines (KFP) locally.")
        .arg(Arg::with_name("skip_confirm")
            .short("-y")
            .long("yes")
            .help("Answers all of the prompts with 'yes', resulting in a no-touch execution."))
        .subcommand(SubCommand::with_name("init")
            .about(&*format!("Ensures the dependencies are met ({}).", Paint::yellow("may need to be run as sudo"))))
        .subcommand(SubCommand::with_name("service")
            .about("Commands to interact with the k3d cluster, and the KFP service.")
            .subcommand(SubCommand::with_name("start")
                .about("Starts the k8s cluster, and the KFP service."))
            .subcommand(SubCommand::with_name("stop")
                .about("Stops the k8s cluster, and the KFP service.")))
        .subcommand(SubCommand::with_name("ui")
            .about("Starts the port forwarding to the KFP UI via `kubectl`.")
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("8080")
                .help("The localhost port to which you want to bind the port forward.")))
        .get_matches();
        
    execute(&args).await?;

    Ok(())
}

async fn execute(args: &ArgMatches<'_>) -> Result<()> {
    let confirm = !args.is_present("skip_confirm");
    let (sub_name, sub_matches) = args.subcommand();

    match sub_name {
        "init" => init(confirm).await,
        "service" => service(confirm, sub_matches.unwrap()).await,
        "ui" => ui(confirm, sub_matches.unwrap()).await,
        _ => Err(Error::msg("Please use a subcommand (check out `kfpl -h` for help)."))
    }
}

async fn init(confirm: bool) -> Result<()> {
    println!("Ensuring proper {} ...", Paint::blue("dependencies"));

    Curl::default().ensure(confirm).await?;
    Git::default().ensure(confirm).await?;
    K3d::default().ensure(confirm).await?;
    Kubectl::default().ensure(confirm).await?;
    Pip3::default().ensure(confirm).await?;
    Kfp::default().ensure(confirm).await?;
    Docker::default().ensure(confirm).await?;

    Ok(())
}

async fn service(confirm: bool, args: &ArgMatches<'_>) -> Result<()> {
    let (sub_name, _) = args.subcommand();

    match sub_name {
        "start" => {
            println!("Ensuring {} are running ...", Paint::blue("services"));
            K3dService::default().ensure(confirm).await?;
            KfpService::default().ensure(confirm).await?;
        },
        "stop" => {
            println!("Stopping {} ...", Paint::blue("services"));
            K3dService::default().remove(confirm).await?;
        },
        _ => return Err(Error::msg("Please use a subcommand (check out `kfpl service -h` for help)."))
    }

    Ok(())
}

async fn ui(confirm: bool, args: &ArgMatches<'_>) -> Result<()> {
    println!("Starting the {} to the KFP UI ...", Paint::blue("port forward"));

    // SAFETY: unwrap is safe because it has a default value.
    let port = args.value_of("port").unwrap();
    
    PortForward::default()
        .with_port(port)
        .ensure(confirm).await?;

    Ok(())
}