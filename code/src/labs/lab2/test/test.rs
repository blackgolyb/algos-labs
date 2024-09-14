#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! binome_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let ((n, k), expected) = $value;
                assert_eq!(expected, binome_iter(n, k));
                assert_eq!(expected, binome_req(n, k));
            }
        )*
        }
    }

    macro_rules! binome_tests_by_formula {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (n, k) = $value;
                assert_eq!(binome_iter(n - 1, k - 1) + binome_iter(n - 1, k), binome_iter(n, k));
                assert_eq!(binome_req(n - 1, k - 1) + binome_req(n - 1, k), binome_req(n, k));
            }
        )*
        }
    }

    binome_tests! {
        c_n_0: ((3, 0), 1),
        c_n_n: ((3, 3), 1),
    }

    binome_tests_by_formula! {
        c_5_2: (5, 2),
        c_7_6: (7, 6),
        c_6_3: (6, 3),
    }
}