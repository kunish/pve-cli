use crate::api::{get_client, get_node, get_nodes, get_qemu, get_qemu_status, get_qemus};
use crate::model::{App, Commands, NodeCommands, QemuCommands};
use clap::Parser;

pub async fn run() -> Result<App, Box<dyn std::error::Error>> {
    let app = App::parse();

    let client = get_client(&app).await?;

    match &app.commands {
        Commands::Nodes {
            node_name,
            commands,
        } => match node_name {
            Some(node_name) => match commands {
                Some(command) => match command {
                    NodeCommands::Qemu { vmid, commands } => match vmid {
                        Some(vmid) => match commands {
                            Some(command) => match command {
                                QemuCommands::Status => {
                                    let qemus =
                                        get_qemu_status(&client, &app, node_name, vmid).await?;
                                }
                            },
                            None => {
                                let qemu = get_qemu(&client, &app, &node_name, &vmid).await?;

                                println!("{}", serde_json::to_string_pretty(&qemu)?);
                            }
                        },
                        None => {
                            let qemus = get_qemus(&client, &app, &node_name).await?;

                            println!("{}", serde_json::to_string_pretty(&qemus)?);
                        }
                    },
                },
                None => {
                    let node = get_node(&client, &app, node_name).await?;

                    println!("{}", serde_json::to_string_pretty(&node)?);
                }
            },
            None => {
                let nodes = get_nodes(&client, &app).await?;

                println!("{}", serde_json::to_string_pretty(&nodes)?);
            }
        },
    }

    Ok(app)
}
