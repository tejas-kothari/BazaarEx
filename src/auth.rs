use marine_rs_sdk::marine;

#[marine]
pub fn get_init_peer_id() -> String {
    let meta = marine_rs_sdk::get_call_parameters();
    meta.init_peer_id
}

pub fn is_owner() -> bool {
    let meta = marine_rs_sdk::get_call_parameters();
    let caller = meta.init_peer_id;
    let owner = meta.service_creator_peer_id;

    caller == owner
}

#[marine]
pub fn am_i_owner() -> bool {
    is_owner()
}
