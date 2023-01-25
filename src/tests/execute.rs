use archway_sdk::test::mock_deps_archway;

use crate::contract::ADMIN;

use super::common::{mock_instantiate, INIT_OWNER};

#[test]
fn instantiate_contract() {
    let mut deps = mock_deps_archway();
    mock_instantiate(deps.as_mut());

    let res = ADMIN.query_admin(deps.as_ref()).unwrap();
    assert_eq!(Some(INIT_OWNER.into()), res.admin);
}
