use crate::day_2::match_outcome::{MatchVsPaper, MatchVsRock, MatchVsScissors};
use crate::day_2::points::Points;
use crate::day_2::shape_type::GetType;

mod match_outcome;
mod paper;
mod points;
mod puzzles;
mod rock;
mod scissor;
mod shape_factory;
mod shape_type;

pub(crate) trait RockPaperScissorTraits:
    MatchVsPaper + MatchVsRock + MatchVsScissors + Points + GetType
{
}
