use crate::distribution::{download_file, install_eula, install_server_jar};
use crate::error::*;
use inquire::Select;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

pub struct Fabric {
    version: String,
    loader: String,
    installer: String,
}

impl Fabric {
    pub async fn new() -> Result<Self> {
        let version_list = Self::get_versions().await?;
        let options = version_list.versions.clone();
        let version = Select::new("Select version", options).prompt()?;

        let loader_list = Self::get_loaders().await?;
        let options = loader_list;
        let loader = Select::new("Select loader", options).prompt()?;

        let installer_list = Self::get_installers().await?;
        let options = installer_list;
        let installer = Select::new("Select installer", options).prompt()?;

        Ok(Self {
            installer: installer.version,
            loader: loader.version,
            version: version.version,
        })
    }

    async fn get_versions() -> Result<VersionList> {
        let url = "https://meta.fabricmc.net/v2/versions/";
        let res = reqwest::get(url).await?;
        let body = res.text().await?;
        let ver = serde_json::from_str(&body)?;
        Ok(ver)
    }

    async fn get_loaders() -> Result<Vec<LoaderInfo>> {
        let url = "https://meta.fabricmc.net/v2/versions/loader";
        let res = reqwest::get(url).await?;
        let body = res.text().await?;
        let ver = serde_json::from_str(&body)?;
        Ok(ver)
    }

    async fn get_installers() -> Result<Vec<InstallerInfo>> {
        let url = "https://meta.fabricmc.net/v2/versions/installer";
        let res = reqwest::get(url).await?;
        let body = res.text().await?;
        let ver = serde_json::from_str(&body)?;
        Ok(ver)
    }

    pub async fn install(&self, path: &PathBuf) -> Result<()> {
        let url = format!(
            "https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar",
            self.version, self.loader, self.installer
        );
        let content = download_file(&url).await?;

        install_server_jar(path, &content).await?;
        install_eula(path).await?;

        Ok(())
    }
}

#[derive(Deserialize, Clone)]
struct VersionList {
    #[serde(rename = "game")]
    versions: Vec<VersionInfo>,
}

#[derive(Deserialize, Clone)]
struct VersionInfo {
    version: String,
    stable: bool,
}

impl Display for VersionInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let stable = match self.stable {
            true => "stable",
            false => "unstable",
        };
        write!(f, "{} - {}", self.version, stable)
    }
}

#[derive(Deserialize, Clone)]
struct LoaderInfo {
    version: String,
    stable: bool,
}

impl Display for LoaderInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let stable = match self.stable {
            true => "stable",
            false => "unstable",
        };
        write!(f, "{} - {}", self.version, stable)
    }
}

#[derive(Deserialize, Clone)]
struct InstallerInfo {
    version: String,
    stable: bool,
}

impl Display for InstallerInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let stable = match self.stable {
            true => "stable",
            false => "unstable",
        };
        write!(f, "{} - {}", self.version, stable)
    }
}
