# pve-cli

Proxmox CLI written in Rust

## Usage

help menu

```shell
pve-cli
```

export required environment variables

```shell
export PVE_ENDPOINT=https://pve.homelab.io:8006/api2/json
export PVE_USERNAME=root@pam
export PVE_PASSWORD=proxmox
```

or pass these variables via command line arguments

```shell
pve-cli -e https://pve.homelab.io:8006/api2/json -u root@pam -p proxmox
```

get node list

```shell
pve-cli nodes
```

get qemu list within a node (pve)

```shell
pve-cli nodes pve qemus
```
