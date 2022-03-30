use mongodb::Database;

use crate::services::Services;

use super::jwt::JWT;

#[derive(Clone)]
pub struct AppDataPool {
    pub database: Database,
    pub services: Services,
    pub jwt: JWT,
}

impl AppDataPool {
    /// Initialize a new AppDataPool
    ///
    /// # Arguments
    ///
    /// * `database` - The `Database` struct that can be used to perform CRUD operations
    /// * `services` - The `Services` struct that contains all available services
    /// * `jwt` - The `JWT` struct that contains JWT configuration
    pub fn new(database: Database, services: Services, jwt: JWT) -> AppDataPool {
        AppDataPool {
            database,
            services,
            jwt,
        }
    }
}
