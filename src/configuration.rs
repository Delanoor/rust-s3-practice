use std::env::set_var;

#[derive(serde::Deserialize)]
pub struct S3Settings {
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}
#[derive(serde::Deserialize)]
pub struct ServerSettings {
    pub port: String,
    pub addr: String,
}

pub fn get_aws_configuration() -> Result<(), config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    set_var("AWS_REGION", &settings.get_string("aws_s3.region")?);
    set_var(
        "AWS_ACCESS_KEY_ID",
        &settings.get_string("aws_s3.access_key")?,
    );
    set_var(
        "AWS_SECRET_ACCESS_KEY",
        &settings.get_string("aws_s3.secret_key")?,
    );

    Ok(())
}

pub fn get_server_configuration() -> Result<ServerSettings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    let server_settings = ServerSettings {
        port: settings.get_string("server.port")?,
        addr: format!("0.0.0.0:{}", settings.get_string("server.port")?),
    };

    Ok(server_settings)
}

