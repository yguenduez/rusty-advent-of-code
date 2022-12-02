use crate::day_2::points::Points;
use crate::day_2::shape_type::{GetType, ShapeType};

#[derive(PartialEq, Debug)]
pub(crate) enum MatchOutCome {
    Win,
    Draw,
    Lose,
}

impl Points for MatchOutCome {
    fn points(&self) -> u64 {
        match self {
            MatchOutCome::Win => 6,
            MatchOutCome::Draw => 3,
            MatchOutCome::Lose => 0,
        }
    }
}

pub(crate) trait MatchVsRock {
    fn vs_rock(&self) -> MatchOutCome;
}

pub(crate) trait MatchVsPaper {
    fn vs_paper(&self) -> MatchOutCome;
}

pub(crate) trait MatchVsScissors {
    fn vs_scissors(&self) -> MatchOutCome;
}

pub(crate) fn outcome_match_in_points<T, U>(first_shape: &T, second_shape: &U) -> u64
where
    T: MatchVsPaper + MatchVsRock + MatchVsScissors + Points + GetType,
    U: MatchVsPaper + MatchVsRock + MatchVsScissors + Points + GetType,
{
    let points_for_shape = first_shape.points();
    let match_outcome = match second_shape.get_type() {
        ShapeType::RockType => first_shape.vs_rock(),
        ShapeType::PaperType => first_shape.vs_paper(),
        ShapeType::ScissorsType => first_shape.vs_scissors(),
    };

    let match_points = match_outcome.points();
    points_for_shape + match_points
}
