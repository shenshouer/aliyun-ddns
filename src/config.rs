use clap::ArgMatches;
use std::env;
use std::str::FromStr;

/// 命令行参数
#[derive(Clone, Debug)]
pub struct Options {
    pub access_key_id: Option<String>,
    pub access_key_secret: Option<String>,
    pub region_id: String,
    pub domains: Vec<String>,
    pub period: u32,
    pub ttl: u32,
    pub mode: Mode,
}

impl Options {
    fn new(mode: Mode) -> Self {
        Options {
            access_key_id: None,
            access_key_secret: None,
            region_id: String::from("cn-hangzhou"),
            domains: Vec::default(),
            period: 600,
            ttl: 600,
            mode: mode,
        }
    }

    /// 通过命令行参数构建
    pub fn from_args(args: &ArgMatches) -> Self {
        let mut options = Options::new(Mode::Cli);

        //从命令行参数内获取
        if let Some(var) = args.value_of(super::OPTION_AKID) {
            options.access_key_id = Some(var.to_string());
        }
        if let Some(var) = args.value_of(super::OPTION_AKSCT) {
            options.access_key_secret = Some(var.to_string());
        }
        if let Some(var) = args.value_of(super::OPTION_DOMAIN) {
            options.domains = Options::sqlit_domain(var);
        }
        if let Some(var) = args.value_of(super::OPTION_PERIOD) {
            options.period = var.parse().unwrap();
        }
        if let Some(var) = args.value_of(super::OPTION_TTL) {
            options.ttl = var.parse().unwrap();
        }
        options
    }

    /// 通过环境变量参数构建
    pub fn from_env() -> Self {
        let mut options = Options::new(Mode::Env);

        //从环境变量内获取, 如存在则覆盖原值
        if let Ok(var) = env::var(super::OPTION_AKID) {
            options.access_key_id = Some(var);
        }
        if let Ok(var) = env::var(super::OPTION_AKSCT) {
            options.access_key_secret = Some(var);
        }
        if let Ok(var) = env::var(super::OPTION_DOMAIN) {
            options.domains = Options::sqlit_domain(&var);
        }
        if let Ok(var) = env::var(super::OPTION_PERIOD) {
            options.period = var.parse().unwrap();
        }
        if let Ok(var) = env::var(super::OPTION_TTL) {
            options.ttl = var.parse().unwrap();
        }
        options
    }

    /// 校验必填参数
    pub fn verify(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn sqlit_domain(domains: &str) -> Vec<String> {
        if domains.contains(",") {
            domains.split(",").map(|s| s.to_string()).collect()
        } else {
            vec![String::from(domains)]
        }
    }
}

#[derive(Clone, Debug)]
pub enum Mode {
    Cli,
    Env,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cli" => Ok(Mode::Cli),
            "env" => Ok(Mode::Env),
            _ => Err("no match"),
        }
    }
}
