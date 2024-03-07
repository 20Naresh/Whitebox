use crate::add;


#[test]
    fn test_add_positive_numbers_integration() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_add_negative_numbersintegration() {
        let result = add(-2, 3);
        assert_eq!(result, 0);
    }    
