///
/// Private CLI common functions.
/// This module contains functions that are common to the CLI binaries.
/// Should not be used directly.
///
use super::consts::REDIS_KEY_TYPE;

#[derive(Clone, Debug)]
pub enum DbOption {
    Db(u32),
    All,
}

pub fn is_number_or_all(s: &str) -> Result<DbOption, String> {
    if s == "all" {
        Ok(DbOption::All)
    } else {
        s.parse::<u32>()
            .map(DbOption::Db)
            .map_err(|_| format!("valid values are: <integer> | all"))
    }
}

pub fn key_type_exists(s: &str) -> Result<String, String> {
    REDIS_KEY_TYPE
        .contains(&s)
        .then(|| s.to_string())
        .ok_or_else(|| {
            format!(
                "Redis key type `{}` is not one of: {}",
                s,
                REDIS_KEY_TYPE.join(", ")
            )
        })
}
