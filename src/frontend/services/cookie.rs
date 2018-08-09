//! A cookie handling service to read and write cookies

use failure::Error;
use stdweb::unstable::TryInto;

#[derive(Debug, Fail)]
pub enum CookieError {
    #[fail(display = "no cookie found")]
    NotFound,
}

pub struct CookieService;

impl CookieService {
    pub fn new() -> Self {
        CookieService
    }

    pub fn set(&self, name: &str, value: &str) {
        self.set_expiring(name, value, 365);
    }

    /// Set a cookie for a given name and value for a default validity of one year
    pub fn set_expiring(&self, name: &str, value: &str, days: i32) {
        js! {
            document.cookie = @{name} + "=" + @{value} + 
                ";max-age=" + (@{days} * 24 * 60 * 60) + ";path=/";
        };
    }

    pub fn get(name: &str) -> Result<String, Error> {
        let cookies = js! { return document.cookie.split(";"); };
        let cookies: Vec<String> = cookies.try_into()?;
        cookies
            .iter()
            .filter_map(|c| {
                let value_pair: Vec<_> = c.split("=").collect();
                match value_pair.get(0) {
                    None => None,
                    Some(name_value) => {
                        if *name_value == name {
                            value_pair.get(1).map(|x| (*x).to_owned())
                        } else { None }
                    }
                }
            }).collect::<Vec<_>>()
                .pop()
                .ok_or(CookieError::NotFound.into())
    }

    pub fn remove(&self, name: &str) {
        self.set_expiring(name, "", -1);
    }
}