use liboxidizer;

#[test]
fn test_connection() {
    liboxidizer::connect("ws://localhost:50000").unwrap();
}
