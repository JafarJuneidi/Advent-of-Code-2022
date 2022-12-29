use nom::{
    self,
    branch::alt,
    character::complete::{self, line_ending},
    combinator::{eof, iterator},
    sequence::terminated,
    IResult,
};

fn numbers(input: &str) -> IResult<&str, Vec<(usize, i64)>> {
    let mut itr = iterator(input, terminated(complete::i64, alt((line_ending, eof))));
    let numbers = itr.enumerate().collect::<Vec<(usize, i64)>>();
    let (input, _) = itr.finish()?;
    Ok((input, numbers))
}

fn part1(input: &str) -> String {
    let (_, numbers) = numbers(input).unwrap();
    let mut state = numbers.clone();
    for (id, _value) in numbers.iter() {
        let index = state
            .iter()
            .position(|state_value| state_value.0 == *id)
            .unwrap();

        let current = state.remove(index);
        let added = index as i64 + current.1;
        let new_index = added.rem_euclid(state.len() as i64);

        state.insert(new_index as usize, current);
    }
    let zero_pos = state.iter().position(|v| v.1 == 0).unwrap();
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;
    (a + b + c).to_string()
}

fn part2(input: &str) -> String {
    let (_, mut numbers) = numbers(input).unwrap();
    numbers.iter_mut().for_each(|tuple| tuple.1 *= 811589153);
    let mut state = numbers.clone();
    for _ in 0..10 {
        for (id, _value) in numbers.iter() {
            let index = state
                .iter()
                .position(|state_value| state_value.0 == *id)
                .unwrap();

            let current = state.remove(index);
            let added = index as i64 + current.1;
            let new_index = added.rem_euclid(state.len() as i64);

            state.insert(new_index as usize, current);
        }
    }

    let zero_pos = state.iter().position(|v| v.1 == 0).unwrap();
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;
    (a + b + c).to_string()
}

fn main() {
    let input = include_str!("./day20.txt");
    let part1 = part1(input);
    let part2 = part2(input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
