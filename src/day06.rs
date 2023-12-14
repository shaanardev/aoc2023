use crate::Solution;

#[derive(Debug)]
pub struct Race {
    times: Vec<i64>,
    distances: Vec<i64>
}
impl Race {
    pub fn cal_ways_to_win(&self) -> Vec<i64> {
        let mut counts: Vec<i64> = Vec::new();
        for (idx, t) in self.times.iter().enumerate() {
            let dist = self.distances[idx];
            let mut count = 0;
            for v in 1..=*t {
                let curr_dist = v * (t - v);
                if curr_dist > dist {
                    count += 1;
                }
            }
            counts.push(count);
        }

        counts
    }
}


#[derive(Debug)]
pub struct Day06;
impl Solution for Day06 {
    type ParsedInput = Vec<Race>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let lines: Vec<String> = input_lines.lines().map(String::from).collect();
        let race_one = parse_for_p1(&lines);
        let race_two = parse_for_p2(&lines);
    
        vec![race_one, race_two]
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let race = &parsed_input[0];

        let counts = race.cal_ways_to_win();

        counts.iter().fold(1, |acc, x| acc * x).to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let race = &parsed_input[1];
        
        let counts = race.cal_ways_to_win();

        counts.iter().fold(1, |acc, x| acc * x).to_string()
    }
}

pub fn parse_for_p1(lines: &Vec<String>) -> Race {
    let times = lines[0].split_once(":").unwrap();
    let times_values: Vec<i64> = times.1.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let dists = lines[1].split_once(":").unwrap();
    let dists_values: Vec<i64> = dists.1.split_whitespace().map(|x| x.parse().unwrap()).collect();

    assert_eq!(times_values.len(), dists_values.len());
    Race { 
        times: times_values, 
        distances: dists_values  
    }
}

pub fn parse_for_p2(lines: &Vec<String>) -> Race {
    let times = lines[0].split_once(":").unwrap();
    let dists = lines[1].split_once(":").unwrap();

    let mut times_val = String::from(times.1);
    times_val.retain(|c| !c.is_whitespace());
    let parsed_time: i64 = times_val.parse().unwrap();

    let mut dist_val = String::from(dists.1);
    dist_val.retain(|c| !c.is_whitespace());
    let parsed_dist: i64 = dist_val.parse().unwrap();

    Race { 
        times: vec![parsed_time], 
        distances: vec![parsed_dist] 
    }
}