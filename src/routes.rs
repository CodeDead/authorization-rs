use actix_web::web;
use jsonwebtoken::{DecodingKey, Validation};
use mongodb::Database;

use crate::configuration::appdatapool::AppDataPool;
use crate::persistence::user::model::user::User;
use crate::services::permission::permission_service::PermissionService;
use crate::services::role::role_service::RoleService;

use self::authentication::authentication_route;
use self::authentication::dto::authentication_response::Claims;
use self::health::health_route;
use self::user::user_route;

pub mod authentication;
pub mod health;
pub mod user;

pub const EMAIL_REGEX_PATTERN: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

pub struct Routes {}

impl Routes {
    pub fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/actuators").service(health_route::get_status));

        cfg.service(
            web::scope("/users")
                .service(user_route::create_user)
                .service(user_route::find_all_users)
                .service(user_route::find_by_uuid)
                .service(user_route::update_by_uuid)
                .service(user_route::update_password)
                .service(user_route::delete_by_uuid),
        );

        cfg.service(
            web::scope("/authentication")
                .service(authentication_route::authenticate)
                .service(authentication_route::register)
                .service(authentication_route::get_current_user),
        );
    }
}

pub async fn check_user_permissions(
    req: &actix_web::HttpRequest,
    pool: &web::Data<AppDataPool>,
    permission_name: &str,
) -> bool {
    let uuid = match get_user_uuid_from_token(req, pool) {
        None => return false,
        Some(d) => d,
    };

    println!("Checking UUID");

    match pool
        .services
        .user_service
        .find_by_uuid(&pool.database, &uuid)
        .await
    {
        Ok(res) => match res {
            None => false,
            Some(d) => {
                if !d.enabled {
                    return false;
                }
                return does_user_have_permission(
                    &pool.database,
                    &d,
                    &pool.services.role_service,
                    &pool.services.permission_service,
                    &permission_name,
                )
                .await;
            }
        },
        Err(_e) => false,
    }
}

pub fn get_user_uuid_from_token(
    req: &actix_web::HttpRequest,
    pool: &web::Data<AppDataPool>,
) -> Option<String> {
    let auth_option = req.headers().get("Authorization");
    let auth = match auth_option {
        None => return None,
        Some(d) => d,
    };

    let token_result = auth.to_str();
    let token = match token_result {
        Ok(d) => String::from(d),
        Err(_e) => return None,
    };

    if token.len() < 8 {
        return None;
    }

    let slice = &token[0..7];
    if slice.to_lowercase() != "bearer " {
        return None;
    }

    let bearer_token = &token[7..token.len()];

    let token_result = jsonwebtoken::decode::<Claims>(
        &bearer_token,
        &DecodingKey::from_secret(&pool.jwt.secret.as_ref()),
        &Validation::default(),
    );
    let res = match token_result {
        Ok(d) => d,
        Err(_e) => return None,
    };

    return Some(res.claims.sub);
}

pub async fn does_user_have_permission(
    db: &Database,
    user: &User,
    role_service: &RoleService,
    permission_service: &PermissionService,
    permission_name: &str,
) -> bool {
    if permission_name.is_empty() {
        return false;
    }

    for role in &user.roles {
        let optional_role = match role_service.find_by_uuid(db, role).await {
            Ok(d) => d,
            Err(_) => return false,
        };

        let actual_role = match optional_role {
            Some(d) => d,
            None => return false,
        };

        for permission in &actual_role.permissions {
            let optional_permission = match permission_service.find_by_uuid(db, permission).await {
                Ok(d) => d,
                Err(_) => return false,
            };

            let actual_permission = match optional_permission {
                Some(d) => d,
                None => return false,
            };

            if actual_permission.name == permission_name {
                return true;
            }
        }
    }
    false
}

pub fn convert_user_to_dto(user: User) -> crate::routes::user::dto::user::User {
    crate::routes::user::dto::user::User {
        id: user.id,
        username: user.username,
        email_address: user.email_address,
        first_name: user.first_name,
        last_name: user.last_name,
        enabled: user.enabled,
        roles: user.roles,
        created_at: user.created_at,
        last_active: user.last_active,
    }
}
