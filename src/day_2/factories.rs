pub(crate) struct ShapeFactory;

use crate::day_2::match_outcome::MatchOutCome;
use crate::day_2::paper::Paper;
use crate::day_2::rock::Rock;
use crate::day_2::scissor::Scissors;
use crate::day_2::shape_type::ShapeType;
use crate::day_2::RockPaperScissorTraits;

impl ShapeFactory {
    pub fn from_str(input: &str) -> Box<dyn RockPaperScissorTraits> {
        match input {
            "A" | "X" => Box::new(Rock),
            "B" | "Y" => Box::new(Paper),
            "C" | "Z" => Box::new(Scissors),
            _ => panic!("Cannot parse input"),
        }
    }
}

pub(crate) struct MatchOutComeFactory;

impl MatchOutComeFactory {
    pub fn from_str(input: &str) -> MatchOutCome {
        match input {
            "X" => MatchOutCome::Lose,
            "Y" => MatchOutCome::Draw,
            "Z" => MatchOutCome::Win,
            _ => panic!("Cannot parse input"),
        }
    }
}

pub(crate) trait ShapeFromOutComeFactory {
    fn other_shape_from_outcome(&self, expected_outcome: MatchOutCome) -> Box<dyn RockPaperScissorTraits>;
}

#[cfg(test)]
mod tests {
    use crate::day_2::factories::MatchOutComeFactory;
    use crate::day_2::match_outcome::MatchOutCome;

    #[test]
    fn correct_match_outcome_from_input() {
        assert_eq!(MatchOutCome::Win, MatchOutComeFactory::from_str("Z"));
        assert_eq!(MatchOutCome::Draw, MatchOutComeFactory::from_str("Y"));
        assert_eq!(MatchOutCome::Lose, MatchOutComeFactory::from_str("X"));
    }
}
