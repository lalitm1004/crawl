pub trait SpatialPayoff {
    fn calculate(
        &self,
        num_rows: usize, num_cols: usize,
        row: usize, col: usize
    ) -> i32;
}

#[cfg(test)]
mod tests {
    use super::SpatialPayoff;

    struct SumPayoff;
    impl SpatialPayoff for SumPayoff {
        fn calculate(
            &self,
            _num_rows: usize, _num_cols: usize,
            row: usize, col: usize
        ) -> i32 {
            (row + col) as i32
        }
    }

    #[test]
    fn test_sum_payoff() {
        let payoff = SumPayoff;
        assert_eq!(payoff.calculate(5, 5, 0, 0), 0);
        assert_eq!(payoff.calculate(5, 5, 2, 3), 5);
        assert_eq!(payoff.calculate(5, 5, 4, 4), 8);
    }

    struct CircleAtCenterPayoff {
        radius_squared: f32,
        inside: i32,
        outside: i32,
    }
    impl CircleAtCenterPayoff {
        fn new(radius: f32, inside: i32, outside: i32) -> Self {
            Self {
                radius_squared: radius.powi(2),
                inside,
                outside
            }
        }
    }
    impl SpatialPayoff for CircleAtCenterPayoff {
        fn calculate(
            &self,
            num_rows: usize, num_cols: usize,
            row: usize, col: usize
        ) -> i32 {
            let center_row = num_rows as f32 / 2.0;
            let center_col = num_cols as f32 / 2.0;

            let dx = center_col - col as f32;
            let dy = center_row - row as f32;
            let distance_squared = dx.powi(2) + dy.powi(2);

            if distance_squared <= self.radius_squared {
                self.inside
            } else {
                self.outside
            }
        }
    }

    #[test]
    fn test_circle_at_center_payoff() {
        let payoff = CircleAtCenterPayoff::new(2.0, 1, 0);

        assert_eq!(payoff.calculate(5, 5, 2, 2), 1);
        assert_eq!(payoff.calculate(5, 5, 0, 0), 0);
    }
}
