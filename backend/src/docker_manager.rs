use shiplift::{Docker, ContainerOptions};
use crate::models::InstanceInfo;

pub async fn start_ubuntu_container() -> Result<InstanceInfo, shiplift::Error> {
    let docker = Docker::new();
    let options = ContainerOptions::builder("ubuntu:latest")
        .tty(true)
        .detach(true)
        .build();
    
    let info = docker.containers().create(&options).await?;
    docker.containers().get(&info.id).start().await?;

    Ok(InstanceInfo {
        id: info.id,
        image: "ubuntu:latest".to_string(),
        status: "running".to_string(),
    })
}

pub async fn list_containers() -> Result<Vec<InstanceInfo>, shiplift::Error> {
    let docker = Docker::new();
    let containers = docker.containers().list(&Default::default()).await?;
    
    let list = containers.into_iter().map(|c| InstanceInfo {
        id: c.id,
        image: c.image,
        status: c.state.unwrap_or("unknown".to_string()),
    }).collect();

    Ok(list)
}
