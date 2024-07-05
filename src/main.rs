use actix_web::{ get, post, web::{ self, ServiceConfig }, HttpResponse, Responder };
use jsonwebtoken::{ Header, encode };
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

use ironback::{
    JWT_ENCODING_KEY,
    JWT_DECODING_KEY,
    Claims,
    Error,
    Result,
    User,
    connect_to_database,
    validate_user,
};

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Create the app state
    let app_state = web::Data::new(AppState::new().await.unwrap());

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(app_state.clone())
            .service(hello_world)
            .service(health_check)
            .service(test_db_connection)
            .service(sign_in)
            .service(validate);
    };

    Ok(config.into())
}

struct AppState {
    db_connection_pool: PgPool,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        // Connect to the database
        let db_connection_pool = connect_to_database().await?;

        Ok(Self { db_connection_pool })
    }
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello from Shuttle!"
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/test-db-connection")]
async fn test_db_connection(app_state: web::Data<AppState>) -> actix_web::Result<HttpResponse> {
    use sqlx::query_scalar;

    // Run a test query
    match query_scalar!("SELECT 1").fetch_one(&app_state.db_connection_pool).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(Error::DatabaseConnection.into()),
    }
}

#[post("/signin")]
async fn sign_in(
    app_state: web::Data<AppState>,
    user: web::Json<User>
) -> actix_web::Result<String> {
    // Validate the user
    match validate_user(&app_state.db_connection_pool, &user).await {
        Ok(is_user_authorized) => if !is_user_authorized {
            return Err(Error::Unauthorized.into());
        }
        Err(error) => {
            return Err(error.into());
        }
    }

    // Get the email
    let email = &user.email;

    // Create the claims
    let claims = Claims::new(email);

    // Create a JWT token
    let token = encode(&Header::default(), &claims, &JWT_ENCODING_KEY).unwrap();

    // Return the token
    Ok(token)
}

#[post("/validate")]
async fn validate(token: String) -> actix_web::Result<HttpResponse> {
    // Create the default validation instance
    let mut validation = jsonwebtoken::Validation::default();

    // Set the leeway to 0 seconds
    validation.leeway = 0;

    // Validate the token
    match jsonwebtoken::decode::<Claims>(&token, &JWT_DECODING_KEY, &validation) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => Err(Error::from(error).into()),
    }
}
