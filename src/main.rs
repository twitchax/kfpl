#![warn(rust_2018_idioms, clippy::all)]

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
    git::Git,
    kfctl::Kfctl,
    kf_service::KfService,
    k9s::K9s
};

// TODO:
//   * Make all of the version, yaml location, blah, blah, blah, configurable.
//   * Pass through options to k3d.
//   * Fix `kfpl service` bug.

#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging.
    SimpleLogger::new().with_level(LevelFilter::Info);

    let init_help = &*format!("Ensures the dependencies are met ({}).", Paint::yellow("may need to be run as sudo"));

    let app = App::new("kfpl")
        .version("1.2.0")
        .author("Aaron Roney")
        .about("Automates running KubeFlow Pipelines (KFP) locally.")
        .arg(Arg::with_name("skip_confirm")
            .short("-y")
            .long("yes")
            .help("Answers all of the prompts with 'yes', resulting in a no-touch execution."))
        .subcommand(SubCommand::with_name("init")
            .about(init_help))
        .subcommand(SubCommand::with_name("service")
            .about("Commands to interact with the k3d cluster, and the KFP service.")
            .subcommand(SubCommand::with_name("start")
                .about("Starts the k8s cluster, and the KFP service.")
                .arg(Arg::with_name("kfp_only")
                    .long("kfp-only")
                    .help("Deploys only KubeFlow Pipelines (KFP), rather than all of KubeFlow."))
                .arg(Arg::with_name("k3d_cluster_name")
                    .short("n")
                    .long("k3d-cluster-name")
                    .takes_value(true)
                    .default_value("kfp-local")
                    .help("The `name` assigned to the cluster created by k3d."))
                .arg(Arg::with_name("k3d_image")
                    .short("i")
                    .long("k3d-image")
                    .takes_value(true)
                    .default_value("rancher/k3s:v1.19.2-k3s1")
                    .help("The `k3s` image used to serve the k3d cluster."))
                .arg(Arg::with_name("k3d_api_address")
                    .short("a")
                    .long("k3d-api-address")
                    .takes_value(true)
                    .default_value("0.0.0.0")
                    .help("The address to which the k3d load balancer for the kubernetes API is bound (e.g., if you don't want outside connections, use `127.0.0.1`)."))
                .arg(Arg::with_name("k3d_api_port")
                    .short("p")
                    .long("k3d-api-port")
                    .takes_value(true)
                    .default_value("6443")
                    .help("The port to which the k3d load balancer for the kubernetes API is bound."))
                .arg(Arg::with_name("kfp_version")
                    .long("kfp-version")
                    .takes_value(true)
                    .default_value("1.0.4")
                    .help("The specific version of KFP to install (only works with the `--kfp-only` option)."))
                .arg(Arg::with_name("kf_yaml")
                    .long("kf-yaml")
                    .takes_value(true)
                    .default_value("https://raw.githubusercontent.com/kubeflow/manifests/v1.1-branch/kfdef/kfctl_k8s_istio.v1.1.0.yaml")
                    .help("The specific YAML manifest used to deploy KF (is ignored when `--kfp-only` is set).")))
            .subcommand(SubCommand::with_name("stop")
                .about("Stops the k8s cluster, and the KFP service.")
                .arg(Arg::with_name("k3d_cluster_name")
                    .short("n")
                    .long("k3d-cluster-name")
                    .takes_value(true)
                    .default_value("kfp-local")
                    .help("The `name` assigned to the cluster created by k3d."))))
        .subcommand(SubCommand::with_name("ui")
            .about("Starts the port forwarding to the KFP UI via `kubectl`.")
            .arg(Arg::with_name("kfp_only")
                .long("kfp-only")
                .help("Port forwards only the KubeFlow Pipelines (KFP) UI, rather than the KubeFlow UI."))
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("8080")
                .help("The localhost port to which you want to bind the port forward."))
            .arg(Arg::with_name("address")
                .short("a")
                .long("address")
                .takes_value(true)
                .default_value("0.0.0.0")
                .help("The address to which the port forwarding proxy is bound (e.g., if you don't want outside connections, use `127.0.0.1`).")));
        
    execute(app).await?;

    println!();

    Ok(())
}

async fn execute(mut app: App<'_, '_>) -> Result<()> {
    let args = app.clone().get_matches();
    let confirm = !args.is_present("skip_confirm");
    let (sub_name, sub_matches) = args.subcommand();

    match sub_name {
        "init" => init(confirm).await,
        "service" => service(confirm, sub_matches.unwrap()).await,
        "ui" => ui(confirm, sub_matches.unwrap()).await,
        _ => app.print_long_help().map_err(|e| e.into())
    }
}

async fn init(confirm: bool) -> Result<()> {
    println!("Ensuring proper {} ...", Paint::blue("dependencies"));

    Curl::default().ensure(confirm).await?;
    Git::default().ensure(confirm).await?;
    K3d::default().ensure(confirm).await?;
    Kubectl::default().ensure(confirm).await?;
    Kfctl::default().ensure(confirm).await?;
    K9s::default().ensure(confirm).await?;
    Pip3::default().ensure(confirm).await?;
    Kfp::default().ensure(confirm).await?;
    Docker::default().ensure(confirm).await?;

    Ok(())
}

async fn service(confirm: bool, args: &ArgMatches<'_>) -> Result<()> {
    let (sub_name, sub_args_option) = args.subcommand();

    let sub_args = sub_args_option.unwrap();

    match sub_name {
        "start" => {
            println!("Ensuring {} are running ...", Paint::blue("services"));
        
            // SAFETY: unwrap is safe because it has a default value.
            let k3d_cluster_name = sub_args.value_of("k3d_cluster_name").unwrap();
            let k3d_image = sub_args.value_of("k3d_image").unwrap();
            let k3d_api_address = sub_args.value_of("k3d_api_address").unwrap();
            let k3d_api_port = sub_args.value_of("k3d_api_port").unwrap();
            let kfp_version = sub_args.value_of("kfp_version").unwrap();
            let kf_yaml = sub_args.value_of("kf_yaml").unwrap();

            K3dService::default()
                .with_k3d_cluster_name(k3d_cluster_name)
                .with_k3d_image(k3d_image)
                .with_k3d_api_address(k3d_api_address)
                .with_k3d_api_port(k3d_api_port)
                .ensure(confirm).await?;

            if sub_args.is_present("kfp_only") {
                KfpService::default()
                    .with_kfp_version(kfp_version)
                    .ensure(confirm).await?;
            } else {
                KfService::default()
                    .with_kf_yaml(kf_yaml)
                    .ensure(confirm).await?;
            }
        },
        "stop" => {
            println!("Stopping {} ...", Paint::blue("services"));

            // SAFETY: unwrap is safe because it has a default value.
            let k3d_cluster_name = sub_args.value_of("k3d_cluster_name").unwrap();

            K3dService::default()
                .with_k3d_cluster_name(k3d_cluster_name)
                .remove(confirm).await?;
        },
        _ => return Err(Error::msg("Please use a subcommand (check out `kfpl service -h` for help)."))
    }

    Ok(())
}

async fn ui(confirm: bool, args: &ArgMatches<'_>) -> Result<()> {
    println!("Starting the {} to the UI ...", Paint::blue("port forward"));

    // SAFETY: unwrap is safe because it has a default value.
    let kfp_only = args.is_present("kfp_only");
    let port = args.value_of("port").unwrap();
    let address = args.value_of("address").unwrap();
    
    PortForward::default()
        .with_kfp_only(kfp_only)
        .with_port(port)
        .with_address(address)
        .ensure(confirm).await?;

    Ok(())
}