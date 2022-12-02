use crate::day_2::match_outcome::{MatchOutCome, MatchVsPaper, MatchVsRock, MatchVsScissors};
use crate::day_2::points::Points;
use crate::day_2::shape_type::{GetType, ShapeType};
use crate::day_2::RockPaperScissorTraits;

pub(crate) struct Paper;

impl Points for Paper {
    fn points(&self) -> u64 {
        2
    }
}

impl GetType for Paper {
    fn get_type(&self) -> ShapeType {
        ShapeType::PaperType
    }
}

impl MatchVsRock for Paper {
    fn vs_rock(&self) -> MatchOutCome {
        MatchOutCome::Win
    }
}

impl MatchVsPaper for Paper {
    fn vs_paper(&self) -> MatchOutCome {
        MatchOutCome::Draw
    }
}

impl MatchVsScissors for Paper {
    fn vs_scissors(&self) -> MatchOutCome {
        MatchOutCome::Lose
    }
}

impl RockPaperScissorTraits for Paper {}

#[cfg(test)]
mod tests {
    use crate::day_2::match_outcome::{MatchOutCome, MatchVsPaper, MatchVsRock, MatchVsScissors};
    use crate::day_2::paper::Paper;
    use crate::day_2::points::Points;
    use crate::day_2::shape_type::{GetType, ShapeType};

    #[test]
    fn correct_type() {
        let sut = Paper;
        assert_eq!(ShapeType::PaperType, sut.get_type());
    }

    #[test]
    fn correct_points() {
        let sut = Paper;
        assert_eq!(2, sut.points());
    }

    #[test]
    fn correct_vs_rock() {
        let sut = Paper;
        assert_eq!(MatchOutCome::Win, sut.vs_rock());
    }

    #[test]
    fn correct_vs_paper() {
        let sut = Paper;
        assert_eq!(MatchOutCome::Draw, sut.vs_paper());
    }

    #[test]
    fn correct_vs_scissors() {
        let sut = Paper;
        assert_eq!(MatchOutCome::Lose, sut.vs_scissors());
    }
}
