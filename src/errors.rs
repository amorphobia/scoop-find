use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScoopFindError {
    #[error("scoop home not found")]
    ScoopHomeNotFound,
    #[error("scoop not installed correctly")]
    ScoopBadInstalled,
    #[error("poinsoned mutex")]
    PoinsonedMutex,
}
