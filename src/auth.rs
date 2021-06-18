use fluence::{fce, CallParameters};

pub fn is_owner() -> bool {
    let meta = fluence::get_call_parameters();
    let caller = meta.init_peer_id;
    let owner = meta.service_creator_peer_id;

    caller == owner
}

#[fce]
pub fn am_i_owner() -> bool {
    is_owner()
}