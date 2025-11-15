pub mod powi;
pub mod schema;
pub mod search;
mod utils;

pub const ZERO_THRESHOLD: f64 = 2.938735877055719e-39;

pub fn go_find_bucket<Search: search::Strategy>(
    schema: impl schema::Schema,
    zero_threshold: f64,
    mut value: f64,
) -> Option<(i32, bool)> {
    let schema = schema.get();
    let mut is_inf = false;
    if value.is_infinite() {
        if value.is_sign_positive() {
            value = f64::MAX;
        } else {
            value = f64::MIN;
        }
        is_inf = true;
    }
    let (frac, exp) = utils::frexp(value.abs());
    let mut key = if schema > 0 {
        let bounds = search::NATIVE_HISTOGRAM_BOUNDS[schema as usize];
        Search::search(bounds, frac) as i32 + (exp - 1) * bounds.len() as i32
    } else {
        let mut key = exp;
        if frac == 0.5 {
            key -= 1;
        }
        let offset = (1i32 << -schema) - 1;
        (key + offset) >> -schema
    };
    if is_inf {
        key += 1;
    }
    match value {
        v if v > zero_threshold => Some((key, true)),
        v if v < -zero_threshold => Some((key, false)),
        _ => None,
    }
}

pub fn find_bucket<Powi: powi::Strategy>(
    schema: impl schema::Schema,
    zero_threshold: f64,
    value: f64,
) -> Option<(i32, bool)> {
    let abs = value.abs();
    if abs <= zero_threshold {
        return None;
    }
    let schema = schema.get();
    let positive = value.is_sign_positive();
    if abs.is_infinite() {
        let max_key = if schema < 0 {
            f64::MAX_EXP >> -schema
        } else {
            f64::MAX_EXP << schema
        };
        return Some((max_key + 1, positive));
    }
    let (frac, exp) = utils::frexp(utils::prev_float(abs));
    if schema > 0 {
        Some((
            utils::frexp(Powi::powi(frac, schema)).1 + (exp << schema),
            positive,
        ))
    } else {
        let offset = (1i32 << -schema) - 1;
        Some(((exp + offset) >> -schema, positive))
    }
}

#[cfg(test)]
mod tests {
    use std::{iter, sync::LazyLock};

    use crate::{find_bucket, go_find_bucket, powi, search, ZERO_THRESHOLD};

    pub(crate) static TEST_VALUES: LazyLock<Vec<f64>> = LazyLock::new(|| {
        [
            0.0,
            0.5,
            2.0,
            0.25f64.next_up(),
            0.25f64.next_down(),
            4.0f64.next_up(),
            4.0f64.next_down(),
            f64::MIN,
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ]
        .into_iter()
        .chain(iter::repeat_with(fastrand::f64).take(1000))
        .collect()
    });

    #[test]
    fn same_results_as_go() {
        for schema in -4..=8 {
            for &value in &*TEST_VALUES {
                assert_eq!(
                    find_bucket::<powi::Naive>(schema, ZERO_THRESHOLD, value),
                    go_find_bucket::<search::Binary>(schema, ZERO_THRESHOLD, value),
                );
            }
        }
    }
}
