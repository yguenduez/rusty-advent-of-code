use crate::day_2::match_outcome::{MatchOutCome, MatchVsPaper, MatchVsRock, MatchVsScissors};
use crate::day_2::points::Points;
use crate::day_2::shape_type::{GetType, ShapeType};

pub(crate) struct Rock;

impl Points for Rock {
    fn points(&self) -> u64 {
        1
    }
}

impl GetType for Rock {
    fn get_type(&self) -> ShapeType {
        ShapeType::RockType
    }
}

impl MatchVsRock for Rock {
    fn vs_rock(&self) -> MatchOutCome {
        MatchOutCome::Draw
    }
}

impl MatchVsPaper for Rock {
    fn vs_paper(&self) -> MatchOutCome {
        MatchOutCome::Lose
    }
}

impl MatchVsScissors for Rock {
    fn vs_scissors(&self) -> MatchOutCome {
        MatchOutCome::Win
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::match_outcome::{MatchOutCome, MatchVsPaper, MatchVsRock, MatchVsScissors};
    use crate::day_2::points::Points;
    use crate::day_2::rock::Rock;
    use crate::day_2::shape_type::{GetType, ShapeType};

    #[test]
    fn correct_type() {
        let sut = Rock;
        assert_eq!(ShapeType::RockType, sut.get_type());
    }

    #[test]
    fn correct_points() {
        let sut = Rock;
        assert_eq!(1, sut.points());
    }

    #[test]
    fn correct_vs_rock() {
        let sut = Rock;
        assert_eq!(MatchOutCome::Draw, sut.vs_rock());
    }

    #[test]
    fn correct_vs_paper() {
        let sut = Rock;
        assert_eq!(MatchOutCome::Lose, sut.vs_paper());
    }

    #[test]
    fn correct_vs_scissors() {
        let sut = Rock;
        assert_eq!(MatchOutCome::Win, sut.vs_scissors());
    }
}
