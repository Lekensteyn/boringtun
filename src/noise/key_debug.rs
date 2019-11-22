use std::io::prelude::Write;

pub fn write_key_for_debug(label: &str, key: &[u8]) -> std::io::Result<()> {
    let filename = std::env::var("WGKEYLOGFILE").or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "WGKEYLOGFILE environment is not set",
    )))?;
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    f.write(format!("{} = {}\n", label, base64::encode(key)).as_bytes())?;
    Ok(())
}
