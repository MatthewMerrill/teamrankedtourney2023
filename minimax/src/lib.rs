use std::cmp::Ordering;

pub mod minimax;
pub mod abmax;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EvalResult {
    FavorOne(u8),
    Evaluate(i32),
    FavorTwo(u8),
}

impl EvalResult {
    pub fn level_up(&self) -> EvalResult {
        match self {
            EvalResult::FavorOne(a) => EvalResult::FavorOne(a + 1),
            EvalResult::FavorTwo(a) => EvalResult::FavorTwo(a + 1),
            f => *f,
        }
    }
}

impl Ord for EvalResult {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // b is "greater" if smaller (winning sooner)
            (EvalResult::FavorOne(a), EvalResult::FavorOne(b)) => b.cmp(a),
            (EvalResult::FavorOne(_), EvalResult::Evaluate(_)) => Ordering::Greater,
            (EvalResult::FavorOne(_), EvalResult::FavorTwo(_)) => Ordering::Greater,

            (EvalResult::Evaluate(_), EvalResult::FavorOne(_)) => Ordering::Less,
            (EvalResult::Evaluate(a), EvalResult::Evaluate(b)) => a.cmp(b),
            (EvalResult::Evaluate(_), EvalResult::FavorTwo(_)) => Ordering::Greater,

            (EvalResult::FavorTwo(_), EvalResult::FavorOne(_)) => Ordering::Less,
            (EvalResult::FavorTwo(_), EvalResult::Evaluate(_)) => Ordering::Less,
            (EvalResult::FavorTwo(a), EvalResult::FavorTwo(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd<EvalResult> for EvalResult {
    fn partial_cmp(&self, other: &EvalResult) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

