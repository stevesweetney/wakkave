use capnp::{
    message::{ Builder, HeapAllocator, ReaderOptions },
    serialize_packed
};
use protocol_capnp::{request, response};

use failure::Error;

#[derive(Debug, Fail)]
pub enum ProtocolError {
    #[fail(display = "Error response: {}", description)]
    Response { description: String },
}

pub struct ProtocolService {
    data: Vec<u8>,
    builder: Builder<HeapAllocator>,
}

impl ProtocolService {
    pub fn new() -> Self {
        ProtocolService {
            data: Vec::new(),
            builder: Builder::new_default(),
        }
    }

    fn write(&mut self) -> Result<&[u8], Error> {
        self.data.clear();

        serialize_packed::write_message(&mut self.data, &self.builder)?;
        Ok(&self.data)
    }

    pub fn write_request_login_credentials(&mut self, name: &str, password: &str) -> Result<&[u8], Error> {
        {
            let mut creds = self
                .builder
                .init_root::<request::Builder>()
                .init_login()
                .init_credentials();
            creds.set_username(name);
            creds.set_password(password);
        }
        
        self.write()
    }

    pub fn read_response_login(&self, mut data: &[u8]) -> Result<Option<String>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::Login(data) => match data.which()? {
                response::login::Token(token) => Ok(Some(token?.to_owned())),
                response::login::Error(error) => Err(Error::from(ProtocolError::Response {
                    description: error?.to_owned()
                })),
            },
            _ => Ok(None),
        } 
    }
}