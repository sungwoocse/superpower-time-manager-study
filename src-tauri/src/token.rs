use std::path::Path;

const INGEST_TOKEN_FILE: &str = "ingest-token";
const INGEST_TOKEN_BYTES: usize = 32;

pub fn load_or_create_ingest_token(app_data_dir: &Path) -> Result<String, std::io::Error> {
    std::fs::create_dir_all(app_data_dir)?;

    let token_path = app_data_dir.join(INGEST_TOKEN_FILE);
    if let Ok(existing) = std::fs::read_to_string(&token_path) {
        let token = existing.trim().to_string();
        if is_valid_ingest_token(&token) {
            return Ok(token);
        }
    }

    let token = generate_ingest_token()?;
    std::fs::write(token_path, &token)?;
    Ok(token)
}

fn generate_ingest_token() -> Result<String, std::io::Error> {
    let mut bytes = [0_u8; INGEST_TOKEN_BYTES];
    getrandom::getrandom(&mut bytes)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;

    Ok(bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>())
}

fn is_valid_ingest_token(token: &str) -> bool {
    token.len() == INGEST_TOKEN_BYTES * 2
        && token.chars().all(|character| character.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_reuses_per_install_ingest_token() {
        let dir = std::env::temp_dir().join(format!(
            "time-manager-token-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();

        let first = load_or_create_ingest_token(&dir).unwrap();
        let second = load_or_create_ingest_token(&dir).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.len(), 64);
        assert!(first.chars().all(|character| character.is_ascii_hexdigit()));

        std::fs::remove_dir_all(dir).unwrap();
    }
}
