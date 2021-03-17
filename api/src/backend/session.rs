use actix_identity::IdentityPolicy;
use actix_session::UserSession;
use actix_web::Error;
use futures::future::{ready, Ready};
use std::rc::Rc;

struct SessionIdentityInner {
    key: String,
}

pub struct SessionIdentiyPolicy(Rc<SessionIdentityInner>);

impl SessionIdentiyPolicy {
    pub fn new() -> SessionIdentiyPolicy {
        SessionIdentiyPolicy(Rc::new(SessionIdentityInner {
            key: "Identity".to_string(),
        }))
    }

    pub fn key(mut self, value: &str) -> SessionIdentiyPolicy {
        Rc::get_mut(&mut self.0).unwrap().key = value.into();
        self
    }
}

impl IdentityPolicy for SessionIdentiyPolicy {
    type Future = Ready<Result<Option<String>, Error>>;

    type ResponseFuture = Ready<Result<(), Error>>;

    fn from_request(&self, req: &mut actix_web::dev::ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let key = &self.0.key;

        ready(session.get::<String>(key).or(Ok(None)))
    }

    fn to_response<B>(
        &self,
        identity: Option<String>,
        changed: bool,
        response: &mut actix_web::dev::ServiceResponse<B>,
    ) -> Self::ResponseFuture {
        let session = response.request().get_session();
        let key = &self.0.key;

        let ret = if changed {
            session.set(key, identity)
        } else {
            Ok(())
        };

        ready(ret)
    }
}
