use crate::add;


#[test]
    fn test_add_positive_numbers_unit() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_add_negative_numbers_unit() {
        let result = add(-2, 3);
        assert_eq!(result, 0);
    }    
