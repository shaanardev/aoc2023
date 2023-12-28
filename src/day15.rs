use regex::Regex;
use crate::Solution;

#[derive(Debug)]
pub struct Step {
    label: String,
    operator: char,
    focal_length: Option<usize>,
}
impl Step {
    fn raw_string(&self) -> String {
        let mut raw_string = self.label.clone();

        raw_string.push(self.operator);

        if let Some(value) = self.focal_length {
            raw_string.push_str(&value.to_string());
        }

        raw_string
    }

    fn apply_operation(&self, boxes: Vec<Vec<Lense>>) -> Vec<Vec<Lense>> {
        match self.operator {
            '-' => self.remove_lense(boxes),
            '=' => self.add_lense(boxes),
            _ => panic!("Unexpected operation {:?}", self.operator),
        }
    }

    fn add_lense(&self, mut boxes: Vec<Vec<Lense>>) -> Vec<Vec<Lense>> {
        let box_idx = hash(&self.label) as usize;

        let new_lense = Lense {
            label: self.label.clone(),
            focal_length: self.focal_length.unwrap(),
        };

        let curr_idx = boxes[box_idx]
            .iter()
            .position(|lense| lense.label == new_lense.label);

        if let Some(lense_index) = curr_idx {
            boxes[box_idx][lense_index] = new_lense;
        } else {
            boxes[box_idx].push(new_lense);
        };

        boxes
    }

    fn remove_lense(&self, mut boxes: Vec<Vec<Lense>>) -> Vec<Vec<Lense>> {
        let box_num = hash(&self.label) as usize;

        boxes[box_num] = boxes[box_num]
            .clone()
            .into_iter()
            .filter(|x| x.label != self.label)
            .collect::<Vec<Lense>>();
        boxes
    }
}

#[derive(Clone, Debug)]
pub struct Lense {
    label: String,
    focal_length: usize,
}

#[derive(Debug)]
pub struct Day15;
impl Solution for Day15 {
    type ParsedInput = Vec<Step>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let re = Regex::new(r"(?<instruction>[a-zA-Z]+)(?<operator>[-=])(?<focal_length>[0-9]*)")
            .unwrap();

        let steps = re.captures_iter(input_lines)
            .map(|cap| {
                let instruction = cap.name("instruction").unwrap().as_str().to_string();
                let operator = cap.name("operator").unwrap().as_str().chars().next().unwrap();
                let raw_focal_length = cap.name("focal_length").unwrap().as_str();
                let focal_length: Option<usize> = if raw_focal_length.is_empty() {
                    None
                } else {
                    Some(raw_focal_length.parse::<usize>().unwrap())
                };
                Step {
                    label: instruction,
                    operator,
                    focal_length,
                }
            })
            .collect::<Vec<Step>>();

        steps
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .map(|step| hash(&step.raw_string()))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(steps: &mut Self::ParsedInput) -> String {
        let mut boxes: Vec<Vec<Lense>> = vec![vec![]; 256];
        for step in steps {
            boxes = step.apply_operation(boxes);
        }
        let mut power = 0;

        for (box_num, my_box) in boxes.iter().enumerate() {
            for (lense_index, lense) in my_box.iter().enumerate() {
                power += (1 + box_num) * (1 + lense_index) * lense.focal_length;
            }
        }

        power.to_string()
    }
}

pub fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.bytes() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }
    h
}

#[cfg(test)]
mod tests {
    use crate::day15::hash;

    #[test]
    fn validate_hash_fn() {
        let val = "HASH";
        assert_eq!(hash(val), 52);
    }
}