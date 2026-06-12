use super::*;

    fn ev(vals: &[u8]) -> Vec<F16Element> {
        vals.iter().map(|&v| F16Element::new(v)).collect()
    }

    #[test]
    fn test_multiply_2x2_with_vector(){

        // A = [[1,2],[3,4]], v = [5,6]
        let a = FieldMatrix::new(2,2,ev(&[1,2,3,4]));
        let v = FieldVector(ev(&[5, 6]));

        // c[0] = 1*5 ^ 2*6 = 9
        // c[1] = 3*5 ^ 4*6 = 4 
        let expected = FieldVector(ev(&[9, 4]));

        assert_eq!(a.multiply_with_vector(v), expected);
    }

    #[test]
    fn test_multiply_2x2_with_2x2() {

        // A = [[1,2],[3,4]], B = [[5,6],[7,8]]
        let a = FieldMatrix::new(2, 2, ev(&[1, 2, 3, 4]));
        let b = FieldMatrix::new(2, 2, ev(&[5, 6, 7, 8]));

        // C = [[11,5],[0,12]]
        let expected = FieldMatrix::new(2, 2, ev(&[11, 5, 0, 12]));

        assert_eq!(a.multiply_with_matrix(b), expected);
    }

    #[test]
    fn test_transpose_2x3() {
        
        // A = [[1,2,3],[4,5,6]] (2x3)
        let a = FieldMatrix::new(2, 3, ev(&[1, 2, 3, 4, 5, 6]));

        // A^T = [[1,4],[2,5],[3,6]] (3x2)
        let expected = FieldMatrix::new(3, 2, ev(&[1, 4, 2, 5, 3, 6]));

        assert_eq!(a.transpose(), expected);
    }