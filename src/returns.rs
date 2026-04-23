// src/returns.rs

pub enum Return {
    Ok(String),
    Err(String),
    NotFound(String),
}
