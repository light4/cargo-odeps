use std::{cmp::Ordering, fmt::Display};

use color_eyre::{eyre::bail, Result};
use semver::Version;
use tracing::debug;
use xshell::{cmd, Shell};

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_HOMEPAGE"),
    ")"
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum KrateStatus {
    #[default]
    Unknown,
    Outdated,
    UpToDate,
    Ignored,
}

impl KrateStatus {
    #[inline]
    pub fn symbol(&self) -> &'static str {
        match self {
            KrateStatus::Unknown => "?",
            KrateStatus::Outdated => "✗",
            KrateStatus::UpToDate => "✓",
            KrateStatus::Ignored => ".",
        }
    }
}

impl Display for KrateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KrateStatus::Unknown => write!(f, "Unknow"),
            KrateStatus::Outdated => write!(f, "Outdated"),
            KrateStatus::UpToDate => write!(f, "UpToDate"),
            KrateStatus::Ignored => write!(f, "Ignored"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DepKrate {
    name: String,
    version: String,
    from: Option<String>,
}

impl DepKrate {
    pub fn get_latest_version(&self) -> Result<String> {
        let versions = get_crate_versions(&self.name)?;
        let krate_json = json::parse(&versions)?;
        let first = &krate_json["versions"][0];

        Ok(first["num"].to_string())
    }

    #[inline]
    pub fn is_local(&self) -> bool {
        if let Some(f) = &self.from {
            return f.starts_with('/');
        }
        false
    }
}

fn get_crate_versions(krate: &str) -> Result<String> {
    let url = format!("https://crates.io/api/v1/crates/{krate}/versions");
    debug!("ureq requesting: {}", &url);
    let body = ureq::get(&url)
        .set("User-Agent", USER_AGENT)
        .call()?
        .into_string()?;

    Ok(body)
}

pub fn get_deps() -> Result<Vec<DepKrate>> {
    let sh = Shell::new()?;
    debug!("running: cargo tree --depth 1");
    let output = cmd!(sh, "cargo tree --depth 1").read()?;
    debug!("cargo tree --depth 1 output:\n {}", &output);
    let result = output
        .lines()
        .skip(1)
        .filter_map(|line| {
            let splited: Vec<&str> = line.trim().split(' ').collect();
            let dep_from = if splited.len() >= 4 {
                let parn: &[_] = &['(', ')'];
                Some(splited[2].trim_matches(parn).to_string())
            } else {
                None
            };
            let d = DepKrate {
                name: splited[1].to_string(),
                version: splited[2].trim_matches('v').to_string(),
                from: dep_from,
            };
            if d.version.is_empty() { None } else { Some(d) }
        })
        .collect();

    Ok(result)
}

#[derive(Debug, Default, Clone)]
pub struct Krate {
    pub dep: DepKrate,
    pub latest: String,
    pub status: KrateStatus,
}

impl Krate {
    #[inline]
    pub fn name(&self) -> &str {
        &self.dep.name
    }

    #[inline]
    pub fn version(&self) -> &str {
        &self.dep.version
    }

    #[inline]
    pub fn is_local(&self) -> bool {
        self.dep.is_local()
    }

    pub fn upgrade(&self) -> Result<()> {
        debug!("upgrading: {}", self.name());
        if KrateStatus::UpToDate == self.status {
            bail!("Already up to date!")
        }
        let sh = Shell::new()?;
        let name = self.name();
        let version = self.version();
        cmd!(sh, "cargo add {name}@{version}").run()?;

        Ok(())
    }
}

pub fn get_all_krates(ignored: &[String], ignore_local: bool) -> Result<Vec<Krate>> {
    let mut result = vec![];
    let deps = get_deps()?;

    for i in &deps {
        let (latest, status) = if ignored.contains(&i.name) || (ignore_local && i.is_local()) {
            let s = KrateStatus::Ignored;
            (s.to_string().to_ascii_lowercase(), s)
        } else if let Ok(v) = i.get_latest_version() {
            let latest = Version::parse(&v).unwrap();
            let current = Version::parse(&i.version).unwrap();
            match latest.cmp(&current) {
                Ordering::Equal => (v, KrateStatus::UpToDate),
                Ordering::Less => (v, KrateStatus::UpToDate),
                Ordering::Greater => (v, KrateStatus::Outdated),
            }
        } else {
            let s = KrateStatus::Unknown;
            (s.to_string().to_ascii_lowercase(), s)
        };
        let krate = Krate {
            dep: i.clone(),
            latest,
            status,
        };
        result.push(krate);
    }

    Ok(result)
}

pub fn print_krates(krates: &[Krate], outdated_only: bool) {
    if outdated_only {
        println!("{:18} {:14} {:14}", "Name", "Dep", "Latest");
    } else {
        println!("{:18} {:14} {:14} {:8}", "Name", "Dep", "Latest", "Status");
    }
    for k in krates {
        if outdated_only {
            if k.status == KrateStatus::Outdated {
                println!("{:18} {:14} {:14}", k.dep.name, k.dep.version, k.latest);
            }
        } else {
            println!(
                "{:18} {:14} {:14} {:8}",
                k.dep.name,
                k.dep.version,
                k.latest,
                k.status.symbol()
            );
        }
    }
}

pub fn upgrade_krates(krates: &[Krate], ignored: &[String], ignore_local: bool) -> Result<()> {
    for k in krates {
        if ignored.contains(&k.name().to_owned()) {
            debug!("ignoring: {}", k.name());
            continue;
        }

        if k.status != KrateStatus::Outdated {
            debug!("ignoring not outdated: {}", k.name());
            continue;
        }

        if ignore_local && k.is_local() {
            debug!("ignoring deps from local: {}", k.name());
            continue;
        }

        println!(
            "Upgrading {} from {} to {} ...",
            k.name(),
            k.version(),
            k.latest
        );
        k.upgrade()?;
    }

    Ok(())
}
