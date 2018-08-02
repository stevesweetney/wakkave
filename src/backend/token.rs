use time;
use uuid::Uuid;
use jsonwebtoken::{self, Header, Validation };
use super::ServerError;
use failure::Error;

lazy_static! {
    static ref SECRET: String = Uuid::new_v4().to_string();
}

/// A web token
#[derive(Serialize, Deserialize)]
pub struct Token {
    /// The subject of the token
    sub: String,
    /// The expiration time of the token
    exp: i64,
    /// The time the token was issued at
    iat: i64,
    /// Unique ID for the token
    jti: String,
}

impl Token {
    pub fn create(username: &str) -> Result<String, Error> {
        const TOKEN_LIFETIME: i64 = 3600;
        let claims = Token {
            sub: username.to_owned(),
            exp: time::get_time().sec + TOKEN_LIFETIME,
            iat: time::get_time().sec,
            jti: Uuid::new_v4().to_string(),
        };

        jsonwebtoken::encode(&Header::default(), &claims, SECRET.as_ref())
            .map_err(|_| Error::from(ServerError::CreateToken))
    }

    pub fn verify(token: &str) -> Result<String, Error> {
        let data = jsonwebtoken::decode::<Token>(
            token, 
            SECRET.as_ref(), 
            &Validation::default()).map_err(|_| Error::from(ServerError::VerifyToken))?;
        Self::create(&data.claims.sub)
    }
}