use crate::configuration::config::Config;

use self::{
    permission::permission_repository::PermissionRepository, role::role_repository::RoleRepository,
    user::user_repository::UserRepository,
};

pub mod permission;
pub mod role;
pub mod user;

#[derive(Clone)]
pub struct Repositories {
    pub user_repository: UserRepository,
    pub role_repository: RoleRepository,
    pub permission_repository: PermissionRepository,
}

impl Repositories {
    pub fn new(config: &Config) -> Repositories {
        Repositories {
            user_repository: UserRepository::new(&config.mongodb.user_collection),
            role_repository: RoleRepository::new(&config.mongodb.role_collection),
            permission_repository: PermissionRepository::new(&config.mongodb.permission_collection),
        }
    }
}
