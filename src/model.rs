use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[clap(author, about, version, propagate_version = true)]
pub struct App {
    #[clap(subcommand)]
    pub commands: Commands,
    #[clap(short, long, env = "PVE_ENDPOINT")]
    pub endpoint: String,
    #[clap(short, long, env = "PVE_USERNAME", hide_env_values(true))]
    pub username: String,
    #[clap(short, long, env = "PVE_PASSWORD", hide_env_values(true))]
    pub password: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Nodes {
        node_name: Option<String>,
        #[clap(subcommand)]
        commands: Option<NodeCommands>,
    },
}

#[derive(Debug, Subcommand)]
pub enum NodeCommands {
    Qemus {
        vmid: Option<String>,
        #[clap(subcommand)]
        commands: Option<QemuCommands>,
    },
}

#[derive(Debug, Subcommand)]
pub enum QemuCommands {
    Status,
}

#[derive(Debug, Subcommand)]
pub enum LxcCommands {}

#[derive(Debug, Deserialize, Serialize)]
pub struct APIResult<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeIndex {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QemuIndex {
    pub subdir: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub node: String,
    pub status: String,
    pub cpu: Option<f64>,
    pub level: Option<String>,
    pub maxcpu: Option<usize>,
    pub maxmem: Option<usize>,
    pub mem: Option<usize>,
    pub ssl_fingerprint: Option<String>,
    pub uptime: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Qemu {
    pub status: String,
    pub vmid: usize,
    pub cpus: f64,
    pub lock: Option<String>,
    pub maxdisk: Option<usize>,
    pub maxmem: Option<usize>,
    pub name: Option<String>,
    pub pid: Option<usize>,
    pub qmpstatus: Option<String>,
    #[serde(rename(deserialize = "running-machine"))]
    pub running_machine: Option<String>,
    #[serde(rename(deserialize = "running-qemu"))]
    pub running_qemu: Option<String>,
    pub tags: Option<String>,
    pub uptime: Option<usize>,
}
