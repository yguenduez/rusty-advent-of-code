pub(crate) struct ShapeFactory;

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
