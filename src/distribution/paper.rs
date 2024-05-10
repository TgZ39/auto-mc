use crate::distribution::download_file;
use crate::error::*;
use bytes::Bytes;
use inquire::Select;
use itertools::Itertools;
use serde::Deserialize;
use strum::Display;

pub struct Paper {
    version: String,
    build_id: i64,
}

impl Paper {
    pub async fn new() -> Result<Self> {
        let version_list = Self::get_versions().await?;

        let mut options = version_list.version_groups;
        options.reverse();
        let group = Select::new("Select version group", options).prompt()?;

        let mut options = version_list.versions;
        options.retain(|v| v.starts_with(&group));
        options.reverse();
        let version = Select::new("Select version", options).prompt()?;

        let build_list = Self::get_builds(&version).await?;

        let options = build_list
            .builds
            .iter()
            .unique_by(|b| b.channel)
            .map(|b| b.channel)
            .collect::<Vec<Channel>>();
        let channel = Select::new("Select channel", options).prompt()?;
        let build_id = build_list
            .builds
            .iter()
            .rfind(|&b| b.channel == channel)
            .unwrap()
            .build_id;

        Ok(Self { version, build_id })
    }

    async fn get_versions() -> Result<VersionList> {
        let url = "https://api.papermc.io/v2/projects/paper";
        let res = reqwest::get(url).await?;
        let body = res.text().await?;
        let ver = serde_json::from_str(&body)?;
        Ok(ver)
    }

    async fn get_builds(version: &str) -> Result<BuildList> {
        let url = format!(
            "https://api.papermc.io/v2/projects/paper/versions/{}/builds",
            version
        );
        let res = reqwest::get(url).await?;
        let body = res.text().await?;
        let builds = serde_json::from_str::<BuildList>(&body)?;

        Ok(builds)
    }

    pub async fn download(&self) -> Result<Bytes> {
        let jar_name = format!("paper-{}-{}.jar", self.version, self.build_id);
        let url = format!(
            "https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/{}",
            self.version, self.build_id, jar_name
        );
        download_file(&url).await
    }
}

#[derive(Deserialize)]
struct VersionList {
    version_groups: Vec<String>,
    versions: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct BuildList {
    builds: Vec<BuildInfo>,
}

#[derive(Deserialize, Debug)]
struct BuildInfo {
    #[serde(rename = "build")]
    build_id: i64,
    channel: Channel,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Display, Copy, Clone, Hash)]
enum Channel {
    #[serde(rename = "experimental")]
    Experimental,
    #[serde(rename = "default")]
    Default,
}
