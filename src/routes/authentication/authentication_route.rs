use actix_web::{get, post, put, web, HttpRequest, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use regex::Regex;
use uuid::Uuid;

use crate::{
    configuration::appdatapool::AppDataPool,
    errors::{bad_request::BadRequest, internal_server_error::InternalServerError},
    persistence::user::model::user::User,
    routes::{
        authentication::dto::{
            authentication_request::AuthenticationRequest,
            authentication_response::{AuthenticationResponse, Claims},
            register_request::RegisterRequest,
            update_request::UpdateRequest,
        },
        convert_user_to_dto, get_user_uuid_from_token,
        user::dto::update_password::UpdatePassword,
        EMAIL_REGEX_PATTERN,
    },
};

#[post("/authenticate")]
pub async fn authenticate(
    pool: web::Data<AppDataPool>,
    login: web::Json<AuthenticationRequest>,
) -> HttpResponse {
    if login.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Username cannot be empty!"));
    }
    if login.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Password cannot be empty!"));
    }

    let user = match pool
        .services
        .user_service
        .find_by_username(&pool.database, &login.username)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let user = match user {
        None => return HttpResponse::Unauthorized().body(""),
        Some(d) => {
            if !d.enabled {
                return HttpResponse::Unauthorized().body("");
            }
            d
        }
    };

    let valid = verify(&login.password, &user.password);
    match valid {
        Ok(d) => {
            if !d {
                return HttpResponse::Unauthorized().body("");
            }
        }
        Err(_) => return HttpResponse::InternalServerError().body(""),
    }

    let res = pool
        .services
        .user_service
        .update_last_active(&pool.database, &user.id, &Utc::now().to_string())
        .await;

    if let Err(e) = res {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    }

    let sub = user.id;
    let iat = Utc::now();
    let exp = iat + chrono::Duration::milliseconds(pool.jwt.expires);

    let claims = Claims::new(sub, iat, exp);

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(pool.jwt.secret.as_ref()),
    );

    match token {
        Ok(res) => HttpResponse::Ok().json(AuthenticationResponse::new(&res)),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[post("/register")]
pub async fn register(
    pool: web::Data<AppDataPool>,
    new_user: web::Json<RegisterRequest>,
) -> HttpResponse {
    if new_user.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Username cannot be empty!"));
    }

    if new_user.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Password cannot be empty!"));
    }

    if new_user.email_address.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Email address cannot be empty!"));
    }

    let email_regex = Regex::new(EMAIL_REGEX_PATTERN).unwrap();
    if !email_regex.is_match(&new_user.email_address) {
        return actix_web::HttpResponse::BadRequest()
            .json(BadRequest::new("Invalid email address!"));
    }

    let optional = match pool
        .services
        .user_service
        .find_by_username(&pool.database, &new_user.username)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    if let Some(_d) = optional {
        return HttpResponse::BadRequest().json(BadRequest::new("Username is already taken!"));
    }

    let optional = match pool
        .services
        .user_service
        .find_by_email_address(&pool.database, &new_user.email_address)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    if let Some(_d) = optional {
        return HttpResponse::BadRequest().json(BadRequest::new("Email address is already taken!"));
    }

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: String::from(&new_user.username),
        email_address: String::from(&new_user.email_address),
        password: hash(&new_user.password, DEFAULT_COST).unwrap(),
        first_name: String::from(&new_user.first_name),
        last_name: String::from(&new_user.last_name),
        enabled: true,
        roles: vec![],
        created_at: Utc::now().to_string(),
        last_active: String::from(""),
    };

    let res = match pool
        .services
        .user_service
        .create(new_user, &pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let res = match res {
        Some(d) => d,
        None => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Unable to create user!"))
        }
    };

    match convert_user_to_dto(
        res,
        &pool.database,
        &pool.services.role_service,
        &pool.services.permission_service,
    )
    .await
    {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[get("/current")]
pub async fn get_current_user(pool: web::Data<AppDataPool>, req: HttpRequest) -> HttpResponse {
    let id = get_user_uuid_from_token(&req, &pool);

    let id = match id {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let user = match pool
        .services
        .user_service
        .find_by_uuid(&pool.database, &id)
        .await
    {
        Ok(d) => match d {
            Some(d) => d,
            None => {
                return HttpResponse::Unauthorized().body("");
            }
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let user = pool
        .services
        .user_service
        .update_last_active(&pool.database, &user.id, &Utc::now().to_string())
        .await;

    let user = match user {
        Ok(d) => match d {
            Some(d) => d,
            None => {
                return HttpResponse::NotFound().body("");
            }
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    match convert_user_to_dto(
        user,
        &pool.database,
        &pool.services.role_service,
        &pool.services.permission_service,
    )
    .await
    {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[put("/current")]
pub async fn update_current_user(
    update: web::Json<UpdateRequest>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    let id = get_user_uuid_from_token(&req, &pool);

    let id = match id {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let res = pool
        .services
        .user_service
        .update_last_active(&pool.database, &id, &Utc::now().to_string())
        .await;

    if let Err(e) = res {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    }

    let mut old_user = match res.unwrap() {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    if !old_user.enabled {
        return HttpResponse::Unauthorized().body("");
    }

    if update.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Username cannot be empty!"));
    }

    if update.email_address.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Email address cannot be empty!"));
    }

    let email_regex = Regex::new(EMAIL_REGEX_PATTERN).unwrap();
    if !email_regex.is_match(&update.email_address) {
        return HttpResponse::BadRequest().json(BadRequest::new("Invalid email address!"));
    }

    let user_by_username = match pool
        .services
        .user_service
        .find_by_username(&pool.database, &update.username)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    if let Some(x) = user_by_username {
        if x.id != old_user.id {
            return HttpResponse::BadRequest().json(BadRequest::new("Username is already taken!"));
        }
    }

    let user_by_email = match pool
        .services
        .user_service
        .find_by_email_address(&pool.database, &update.email_address)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    if let Some(x) = user_by_email {
        if x.id != old_user.id {
            return HttpResponse::BadRequest()
                .json(BadRequest::new("Email address is already taken!"));
        }
    }

    old_user.username = update.username.clone();
    old_user.email_address = update.email_address.clone();
    old_user.first_name = update.first_name.clone();
    old_user.last_name = update.last_name.clone();

    let user = match pool
        .services
        .user_service
        .update(&pool.database, &id, old_user)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    let user = match user {
        Some(d) => d,
        None => {
            return HttpResponse::NoContent().body("");
        }
    };

    match convert_user_to_dto(
        user,
        &pool.database,
        &pool.services.role_service,
        &pool.services.permission_service,
    )
    .await
    {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[put("/current/password")]
pub async fn update_current_user_password(
    update: web::Json<UpdatePassword>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    let id = get_user_uuid_from_token(&req, &pool);

    let id = match id {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    if update.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Password cannot be empty!"));
    }

    match pool
        .services
        .user_service
        .find_by_uuid(&pool.database, &id)
        .await
    {
        Ok(d) => {
            if d.is_none() {
                return HttpResponse::NotFound().body("");
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    let new_password = hash(&update.password, DEFAULT_COST).unwrap();

    let user = match pool
        .services
        .user_service
        .update_password(&pool.database, &id, &new_password)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    let user = match user {
        Some(d) => d,
        None => {
            return HttpResponse::NoContent().body("");
        }
    };

    match convert_user_to_dto(
        user,
        &pool.database,
        &pool.services.role_service,
        &pool.services.permission_service,
    )
    .await
    {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}
