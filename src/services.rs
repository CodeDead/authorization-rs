use crate::{configuration::config::Config, persistence::Repositories};

use self::{
    permission::permission_service::PermissionService, role::role_service::RoleService,
    user::user_service::UserService,
};

pub mod permission;
pub mod role;
pub mod user;

#[derive(Clone)]
pub struct Services {
    pub permission_service: PermissionService,
    pub role_service: RoleService,
    pub user_service: UserService,
}

impl Services {
    pub fn new(config: &Config) -> Services {
        let repositories = Repositories::new(config);

        Services {
            user_service: UserService::new(repositories.user_repository),
            permission_service: PermissionService::new(repositories.permission_repository),
            role_service: RoleService::new(repositories.role_repository),
        }
    }
}
