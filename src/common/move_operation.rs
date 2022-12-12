use regex::{Match, Regex};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub(crate) struct MoveOperation {
    from_stack_index: usize,
    to_stack_index: usize,
    num_of_items: u64,
}

pub(crate) struct MoveOperationFactory {
    move_operation: MoveOperation,
}

impl FromStr for MoveOperation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d)").unwrap();
        let matches: Vec<Match> = re.find_iter(s).collect();
        if matches.len() != 3 {
            return Err(());
        }

        let move_operation = MoveOperationFactory::new()
            .from(matches[1].as_str().parse::<usize>().unwrap())
            .to(matches[2].as_str().parse::<usize>().unwrap())
            .num_of_items_to_move(matches[0].as_str().parse::<u64>().unwrap())
            .build();

        Ok(move_operation)
    }
}

impl MoveOperationFactory {
    fn new() -> Self {
        Self {
            move_operation: MoveOperation {
                from_stack_index: 0,
                to_stack_index: 0,
                num_of_items: 0,
            },
        }
    }

    fn from(mut self, stack: usize) -> Self {
        self.move_operation.from_stack_index = stack;
        self
    }

    fn to(mut self, stack: usize) -> Self {
        self.move_operation.to_stack_index = stack;
        self
    }

    fn num_of_items_to_move(mut self, num_of_items: u64) -> Self {
        self.move_operation.num_of_items = num_of_items;
        self
    }

    pub fn build(self) -> MoveOperation {
        self.move_operation
    }
}

#[cfg(test)]
mod tests {
    use crate::common::move_operation::{MoveOperation, MoveOperationFactory};
    use std::str::FromStr;

    #[test]
    fn builder_pattern_returns_correct_move_operation() {
        let expected_move_operation = MoveOperation {
            from_stack_index: 3,
            to_stack_index: 5,
            num_of_items: 6,
        };

        let result = MoveOperationFactory::new()
            .from(3)
            .to(5)
            .num_of_items_to_move(6)
            .build();
        assert_eq!(expected_move_operation, result);
    }

    #[test]
    fn parses_move_operation_correctly_from_string() {
        let expected_move_operation = MoveOperation {
            from_stack_index: 3,
            to_stack_index: 5,
            num_of_items: 6,
        };

        let result = MoveOperation::from_str("move 6 from 3 to 5").unwrap();
        assert_eq!(expected_move_operation, result);
    }
}
