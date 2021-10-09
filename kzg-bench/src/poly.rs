#[cfg(test)]
mod tests {
    use kzg::{Error, Fr, Poly};

    #[test]
    fn create_poly_of_length_ten() {
        let mut poly = match Poly::new(10) {
            Ok(p) => p,
            Err(_) => Poly::default()
        };
        assert_eq!(poly.length, 10);
        poly.destroy();
    }

    #[test]
    fn create_divided_poly_in_finite_field() {
        let errors = Poly::divide_in_finite_field(15);
        assert_eq!(errors, Error::KzgOk);
    }

    #[test]
    fn poly_eval_check() {
        let n: u64 = 10;
        let mut poly = match Poly::new(n) {
            Ok(p) => p,
            Err(_) => Poly::default()
        };
        for i in 0..n {
            let fr = Fr::from_u64(i + 1);
            poly.set_coeff_at(i as isize, fr);
        }
        let expected = Fr::from_u64(n * (n + 1) / 2);
        let actual = poly.eval_at(&Fr::one());
        assert_eq!(Fr::is_equal(expected, actual), true);
        poly.destroy();
    }

    #[test]
    fn poly_eval_0_check() {
        let n: u64 = 7;
        let a: u64 = 597;
        let mut poly = match Poly::new(n) {
            Ok(p) => p,
            Err(_) => Poly::default()
        };
        for i in 0..n {
            let fr = Fr::from_u64(i + a);
            poly.set_coeff_at(i as isize, fr);
        }
        let expected = Fr::from_u64(a);
        let actual = poly.eval_at(&Fr::zero());
        assert_eq!(Fr::is_equal(expected, actual), true);
        poly.destroy();
    }

    #[test]
    fn poly_eval_nil_check() {
        let n: u64 = 0;
        let mut poly = match Poly::new(n) {
            Ok(p) => p,
            Err(_) => Poly::default()
        };
        let actual = poly.eval_at(&Fr::one());
        assert_eq!(Fr::is_equal(Fr::zero(), actual), true);
        poly.destroy();
    }
}
