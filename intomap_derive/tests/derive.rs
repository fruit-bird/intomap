use intomap_derive::IntoMap;

#[derive(IntoMap)]
#[allow(unused)]
struct User {
    name: &'static str,
    #[intomap(ignore)]
    id: usize,
    #[intomap(rename = "online")]
    active: bool,
}

#[test]
fn intomap_test() {
    let user = User {
        name: "Jimothy",
        id: 0,
        active: true,
    };

    let user_map = user.as_map();
    let should_match = BTreeMap::from([
        ("name".to_string(), "Jimothy".to_string()),
        ("online".to_string(), "true".to_string()),
    ]);

    assert_eq!(user_map, should_match);
}
