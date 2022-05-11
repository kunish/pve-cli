use crate::model::{APIResult, App, Node, NodeIndex, Qemu, QemuIndex};
use reqwest::Client;
use serde_json::json;

pub async fn get_client(app: &App) -> Result<Client, Box<dyn std::error::Error>> {
    let ticket_resp = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?
        .post(format!("{}/access/ticket", app.endpoint))
        .json::<serde_json::Value>(&json!({
            "username": app.username,
            "password": app.password,
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let cookie = format!(
        "PVEAuthCookie={}",
        ticket_resp["data"]["ticket"].as_str().unwrap()
    );
    let csrf_prevention_token = ticket_resp["data"]["CSRFPreventionToken"].as_str().unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Cookie", cookie.parse()?);
    headers.insert("CSRFPreventionToken", csrf_prevention_token.parse()?);

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .build()?;

    Ok(client)
}

pub async fn get_node(
    client: &Client,
    app: &App,
    node_name: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(client
        .get(format!("{}/nodes/{}", app.endpoint, node_name))
        .send()
        .await?
        .json::<APIResult<Vec<NodeIndex>>>()
        .await?
        .data
        .iter()
        .map(|x| x.name.clone())
        .collect())
}

pub async fn get_nodes(
    client: &Client,
    app: &App,
) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
    Ok(client
        .get(format!("{}/nodes", app.endpoint))
        .send()
        .await?
        .json::<APIResult<Vec<Node>>>()
        .await?
        .data)
}

pub async fn get_qemu(
    client: &Client,
    app: &App,
    node_name: &String,
    vmid: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(client
        .get(format!(
            "{}/nodes/{}/qemu/{}",
            app.endpoint, node_name, vmid
        ))
        .send()
        .await?
        .json::<APIResult<Vec<QemuIndex>>>()
        .await?
        .data
        .iter()
        .map(|x| x.subdir.clone())
        .collect())
}

pub async fn get_qemus(
    client: &Client,
    app: &App,
    node_name: &String,
) -> Result<Vec<Qemu>, Box<dyn std::error::Error>> {
    Ok(client
        .get(format!("{}/nodes/{}/qemu", app.endpoint, node_name))
        .send()
        .await?
        .json::<APIResult<Vec<Qemu>>>()
        .await?
        .data)
}

pub async fn get_qemu_status(
    client: &Client,
    app: &App,
    node_name: &String,
    vmid: &String,
) -> Result<Vec<Qemu>, Box<dyn std::error::Error>> {
    Ok(client
        .get(format!(
            "{}/nodes/{}/qemu/{}/status",
            app.endpoint, node_name, vmid
        ))
        .send()
        .await?
        .json::<APIResult<Vec<Qemu>>>()
        .await?
        .data)
}
