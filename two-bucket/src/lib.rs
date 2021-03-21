use crate::Bucket::{One, Two};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    pub fn solve(
        capacity_1: i32,
        capacity_2: i32,
        goal: i32,
        state: (i32, i32),
        forbidden_states: &mut Vec<(i32, i32)>,
        moves: u8,
    ) -> Option<BucketStats> {
        if state.0 == goal {
            Some(BucketStats {
                goal_bucket: One,
                moves,
                other_bucket: state.1 as u8,
            })
        } else if state.1 == goal {
            Some(BucketStats {
                goal_bucket: Two,
                moves,
                other_bucket: state.0 as u8,
            })
        } else if forbidden_states.contains(&state) {
            None
        } else {
            forbidden_states.push(state);
            let mut next_move = |s1, s2| {
                solve(
                    capacity_1,
                    capacity_2,
                    goal,
                    (s1, s2),
                    forbidden_states,
                    moves + 1,
                )
            };

            let res = vec![
                next_move(
                    0.max(state.0 - (capacity_2 - state.1)),
                    capacity_2.min(state.0 + state.1),
                ),
                next_move(
                    capacity_1.min(state.1 + state.0),
                    0.max(state.1 - (capacity_1 - state.0)),
                ),
                next_move(0, state.1),
                next_move(state.0, 0),
                next_move(capacity_1, state.1),
                next_move(state.0, capacity_2),
            ]
            .into_iter()
            .filter_map(|x| x)
            .min_by(|x, y| x.moves.cmp(&y.moves));
            forbidden_states.pop();
            res
        }
    }

    let capacity_1 = capacity_1 as i32;
    let capacity_2 = capacity_2 as i32;
    let state = match start_bucket {
        Bucket::One => (capacity_1, 0i32),
        Bucket::Two => (0i32, capacity_2),
    };
    let mut forbidden_states: Vec<(i32, i32)> = Vec::new();
    forbidden_states.push(((capacity_1 - state.0), (capacity_2 - state.1)));

    solve(
        capacity_1,
        capacity_2,
        goal as i32,
        state,
        &mut forbidden_states,
        1,
    )
}
