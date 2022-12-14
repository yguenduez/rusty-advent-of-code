use crate::day_2::factories::ShapeFromOutComeFactory;
use crate::day_2::match_outcome::{MatchOutCome, MatchVsPaper, MatchVsRock, MatchVsScissors};
use crate::day_2::paper::Paper;
use crate::day_2::points::Points;
use crate::day_2::rock::Rock;
use crate::day_2::shape_type::{GetType, ShapeType};
use crate::day_2::RockPaperScissorTraits;

pub(crate) struct Scissors;

impl Points for Scissors {
    fn points(&self) -> u64 {
        3
    }
}

impl GetType for Scissors {
    fn get_type(&self) -> ShapeType {
        ShapeType::ScissorsType
    }
}

impl MatchVsRock for Scissors {
    fn vs_rock(&self) -> MatchOutCome {
        MatchOutCome::Lose
    }
}

impl MatchVsPaper for Scissors {
    fn vs_paper(&self) -> MatchOutCome {
        MatchOutCome::Win
    }
}

impl MatchVsScissors for Scissors {
    fn vs_scissors(&self) -> MatchOutCome {
        MatchOutCome::Draw
    }
}

impl ShapeFromOutComeFactory for Scissors {
    fn needed_shape_in_order_to(
        &self,
        expected_outcome: MatchOutCome,
    ) -> Box<dyn RockPaperScissorTraits> {
        match expected_outcome {
            MatchOutCome::Win => Box::new(Rock),
            MatchOutCome::Draw => Box::new(Scissors),
            MatchOutCome::Lose => Box::new(Paper),
        }
    }
}

impl RockPaperScissorTraits for Scissors {}

#[cfg(test)]
mod tests {
    use crate::day_2::match_outcome::{MatchOutCome, MatchVsPaper, MatchVsRock, MatchVsScissors};
    use crate::day_2::points::Points;
    use crate::day_2::scissor::Scissors;
    use crate::day_2::shape_type::{GetType, ShapeType};

    #[test]
    fn correct_type() {
        let sut = Scissors;
        assert_eq!(ShapeType::ScissorsType, sut.get_type());
    }

    #[test]
    fn correct_points() {
        let sut = Scissors;
        assert_eq!(3, sut.points());
    }

    #[test]
    fn correct_vs_rock() {
        let sut = Scissors;
        assert_eq!(MatchOutCome::Lose, sut.vs_rock());
    }

    #[test]
    fn correct_vs_paper() {
        let sut = Scissors;
        assert_eq!(MatchOutCome::Win, sut.vs_paper());
    }

    #[test]
    fn correct_vs_scissors() {
        let sut = Scissors;
        assert_eq!(MatchOutCome::Draw, sut.vs_scissors());
    }
}
