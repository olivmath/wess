use crate::server::request::WRequest;

/// # Write Job Type
pub struct WJob {
    pub wreq: WRequest,
    pub wtype: WOps,
    pub id: String,
}

impl WJob {
    pub fn new(wreq: WRequest, wtype: WOps, id: String) -> Self {
        Self { wreq, wtype, id }
    }
}

/// # Write Operation Type
pub enum WOps {
    Create,
    Update,
    Delete,
}
