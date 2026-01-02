use crate::{
    business::server::connection_establisher::ConnectionEstablisher,
    model::request::registration_request::RegistrationRequest,
};

pub trait Restartable<T: ConnectionEstablisher> {
    type Instance;

    fn restart(
        self,
        connection_establisher: &T,
        registration_request: RegistrationRequest,
    ) -> impl Future<Output = Self::Instance>;
}
