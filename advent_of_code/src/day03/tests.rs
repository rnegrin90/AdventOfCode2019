use super::get_cable_instructions;

#[test]
fn test_get_cable_instructions() {
    let output = get_cable_instructions(&String::from("U29,L49"));
    
    assert_eq!(output[0].0, "U");
    assert_eq!(output[0].1, 29);
    assert_eq!(output[1].0, "L");
    assert_eq!(output[1].1, 49);
}