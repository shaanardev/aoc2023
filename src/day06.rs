use crate::Solution;

#[derive(Debug)]
pub struct Races {
    times: Vec<i64>,
    distances: Vec<i64>
}


#[derive(Debug)]
pub struct Day06;
impl Solution for Day06 {
    type ParsedInput = Races;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let lines: Vec<String> = input_lines.lines().map(String::from).collect();
        
        let times = lines[0].split_once(":").unwrap();
        let times_values: Vec<i64> = times.1.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let dists = lines[1].split_once(":").unwrap();
        let dists_values: Vec<i64> = dists.1.split_whitespace().map(|x| x.parse().unwrap()).collect();

        assert_eq!(times_values.len(), dists_values.len());
        Races { 
            times: times_values, 
            distances: dists_values  
        }
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let mut counts: Vec<i64> = Vec::new();
        for (idx, t) in parsed_input.times.iter().enumerate() {
            let dist = parsed_input.distances[idx];
            let mut count = 0;
            for v in 1..=*t {
                let curr_dist = v * (t - v);
                if curr_dist > dist {
                    count += 1;
                }
            }
            counts.push(count);
        }

        counts.iter().fold(1, |acc, x| acc * x).to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        "".to_string()
    }
}