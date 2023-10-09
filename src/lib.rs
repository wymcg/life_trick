mod gol;
mod plugin_fns;
mod util;

#[cfg(test)]
mod util_tests {
    mod neighboring_coords {
        use crate::util::get_neighboring_coordinates;
        use std::collections::HashSet;

        #[test]
        fn neighboring_coordinates_no_overflow() {
            assert_eq!(
                get_neighboring_coordinates((1, 1), 8, 8),
                HashSet::from([
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (0, 1),
                    (2, 1),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                ])
            );
        }

        #[test]
        fn neighboring_coordinates_horizontal_overflow() {
            assert_eq!(
                get_neighboring_coordinates((7, 5), 8, 8),
                HashSet::from([
                    (6, 6),
                    (7, 6),
                    (0, 6),
                    (6, 5),
                    (0, 5),
                    (6, 4),
                    (7, 4),
                    (0, 4),
                ])
            );
        }

        #[test]
        fn neighboring_coordinates_horizontal_underflow() {
            assert_eq!(
                get_neighboring_coordinates((0, 2), 8, 8),
                HashSet::from([
                    (1, 1),
                    (7, 1),
                    (0, 1),
                    (1, 2),
                    (7, 2),
                    (1, 3),
                    (7, 3),
                    (0, 3),
                ])
            );
        }

        #[test]
        fn neighboring_coordinates_vertical_overflow() {
            assert_eq!(
                get_neighboring_coordinates((5, 0), 8, 8),
                HashSet::from([
                    (4, 7),
                    (5, 7),
                    (6, 7),
                    (4, 0),
                    (6, 0),
                    (4, 1),
                    (5, 1),
                    (6, 1),
                ])
            );
        }

        #[test]
        fn neighboring_coordinates_vertical_underflow() {
            assert_eq!(
                get_neighboring_coordinates((2, 7), 8, 8),
                HashSet::from([
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (1, 7),
                    (3, 7),
                    (1, 6),
                    (2, 6),
                    (3, 6),
                ])
            );
        }

        #[test]
        fn neighboring_coordinates_corner() {
            assert_eq!(
                get_neighboring_coordinates((0, 0), 8, 8),
                HashSet::from([
                    (1, 0),
                    (0, 1),
                    (1, 1),
                    (1, 7),
                    (7, 1),
                    (7, 0),
                    (7, 7),
                    (0, 7)
                ])
            );
        }
    }
}

#[cfg(test)]
mod life_tests {
    use crate::gol::GameOfLife;

    fn with_expected_states(expected_states: Vec<Vec<Vec<bool>>>, width: usize, height: usize, should_see_cycle: bool) {
        let mut gol = GameOfLife::new(expected_states[0].clone(), width, height);

        for expected_state_idx in 1..expected_states.len() {
            assert_eq!(gol.advance(), expected_states[expected_state_idx]);
        }

        assert_eq!(gol.is_cycling, should_see_cycle);
    }

    #[test]
    fn blinker() {
        let expected_states = vec![
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, true, true, true, false],
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, true, true, true, false],
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, false, false, false],
            ],
        ];

        with_expected_states(expected_states, 5, 5, true);
    }

    #[test]
    fn block() {
        let expected_states = vec![
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, false, true, true, false],
                vec![false, false, true, true, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, false, true, true, false],
                vec![false, false, true, true, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, false, true, true, false],
                vec![false, false, true, true, false],
                vec![false, false, false, false, false],
            ],
        ];

        with_expected_states(expected_states, 5, 5, true);
    }

    #[test]
    fn glider() {
        let expected_states = vec![
            vec![
                vec![false, false, false, false, false],
                vec![false, false, true, false, false],
                vec![true, false, true, false, false],
                vec![false, true, true, false, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, true, false, false, false],
                vec![false, false, true, true, false],
                vec![false, true, true, false, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, true, false, false],
                vec![false, false, false, true, false],
                vec![false, true, true, true, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, true, false, true, false],
                vec![false, false, true, true, false],
                vec![false, false, true, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, false, false, true, false],
                vec![false, true, false, true, false],
                vec![false, false, true, true, false],
            ],
        ];

        with_expected_states(expected_states, 5, 5, false);
    }
}
