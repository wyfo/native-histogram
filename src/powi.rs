pub trait Strategy {
    fn powi(f: f64, schema: isize) -> f64;
}

pub struct Naive;
impl Strategy for Naive {
    fn powi(f: f64, schema: isize) -> f64 {
        f.powi(1 << schema)
    }
}

pub struct Match;
impl Strategy for Match {
    fn powi(f: f64, schema: isize) -> f64 {
        match schema {
            0 => f.powi(1 << 0),
            1 => f.powi(1 << 1),
            2 => f.powi(1 << 2),
            3 => f.powi(1 << 3),
            4 => f.powi(1 << 4),
            5 => f.powi(1 << 5),
            6 => f.powi(1 << 6),
            7 => f.powi(1 << 7),
            _ => f.powi(1 << 8),
        }
    }
}

pub struct Loop;
impl Strategy for Loop {
    fn powi(mut f: f64, schema: isize) -> f64 {
        for _ in 0..schema {
            f *= f;
        }
        f
    }
}

pub struct BoundedLoop;
impl Strategy for BoundedLoop {
    fn powi(mut f: f64, schema: isize) -> f64 {
        assert!(schema <= 8);
        for _ in 0..schema {
            f *= f;
        }
        f
    }
}

pub struct UnrolledLoop;
impl Strategy for UnrolledLoop {
    fn powi(mut f: f64, schema: isize) -> f64 {
        if schema == 0 {
            return f;
        }
        f *= f;
        if schema == 1 {
            return f;
        }
        f *= f;
        if schema == 2 {
            return f;
        }
        f *= f;
        if schema == 3 {
            return f;
        }
        f *= f;
        if schema == 4 {
            return f;
        }
        f *= f;
        if schema == 5 {
            return f;
        }
        f *= f;
        if schema == 6 {
            return f;
        }
        f *= f;
        if schema == 7 {
            return f;
        }
        f * f
    }
}

pub struct NestedLoops;
impl Strategy for NestedLoops {
    #[allow(clippy::never_loop)]
    fn powi(mut f: f64, schema: isize) -> f64 {
        'l0: loop {
            'l1: loop {
                'l2: loop {
                    'l3: loop {
                        'l4: loop {
                            'l5: loop {
                                'l6: loop {
                                    'l7: loop {
                                        match schema {
                                            0 => break 'l0,
                                            1 => break 'l1,
                                            2 => break 'l2,
                                            3 => break 'l3,
                                            4 => break 'l4,
                                            5 => break 'l5,
                                            6 => break 'l6,
                                            7 => break 'l7,
                                            _ => {}
                                        }
                                        f *= f;
                                        break;
                                    }
                                    f *= f;
                                    break;
                                }
                                f *= f;
                                break;
                            }
                            f *= f;
                            break;
                        }
                        f *= f;
                        break;
                    }
                    f *= f;
                    break;
                }
                f *= f;
                break;
            }
            f *= f;
            break;
        }
        f
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TEST_VALUES;

    #[test]
    fn powi_strategies() {
        use crate::powi::*;
        for schema in 0..=8 {
            for &value in &*TEST_VALUES {
                let powi = Naive::powi(value, schema);
                assert_eq!(Match::powi(value, schema), powi);
                assert_eq!(Loop::powi(value, schema), powi);
                assert_eq!(BoundedLoop::powi(value, schema), powi);
                assert_eq!(UnrolledLoop::powi(value, schema), powi);
                assert_eq!(NestedLoops::powi(value, schema), powi);
            }
        }
    }
}
