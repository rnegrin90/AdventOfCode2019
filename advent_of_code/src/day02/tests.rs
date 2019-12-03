use super::evaluate_memory;

#[test]
fn test_evaluate_memory() {
    assert_eq!(evaluate_memory(&mut vec![1,0,0,0,99]), 2);
    assert_eq!(evaluate_memory(&mut vec![2,3,0,3,99]), 2);
    assert_eq!(evaluate_memory(&mut vec![2,4,4,5,99,0]), 2);
    assert_eq!(evaluate_memory(&mut vec![1,1,1,4,99,5,6,0,99]), 30);
    assert_eq!(evaluate_memory(&mut vec![1,9,10,3,2,3,11,0,99,30,40,50]), 3500);
}
