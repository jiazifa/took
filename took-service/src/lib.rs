use lazy_static::lazy_static;


pub struct ServiceContext {
}

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
        }
    }
}

lazy_static!{
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}