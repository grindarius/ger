use deadpool_postgres::Pool;

/// Data that is shared between each api calls.
pub struct SharedAppData {
    pub pool: Pool,
}

impl SharedAppData {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
