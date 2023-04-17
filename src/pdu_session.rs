use std::net::IpAddr;
use url::Url;

use crate::user::User;

#[derive(Debug, PartialEq, Clone)]
pub struct PDUSession {
    user: User,
    ip_address: IpAddr,
}

impl PDUSession {
    pub fn new(user: User, ip_address: IpAddr) -> Self {
        PDUSession { user, ip_address }
    }

    pub fn release(self) -> (User, IpAddr) {
        return (self.user, self.ip_address);
    }

    pub fn use_application(&self, url: Url) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn create_release() {
        let user = User::new(1);
        let ip_address = Ipv4Addr::LOCALHOST;

        let pbu_session = PDUSession::new(user, std::net::IpAddr::V4(ip_address));
        let (user_1, ip_address_1) = pbu_session.release();
        assert_eq!(User::new(1), user_1);
        assert_eq!(Ipv4Addr::LOCALHOST, ip_address_1);
    }
}
