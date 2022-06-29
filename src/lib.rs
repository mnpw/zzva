pub mod board;
pub mod game;

#[cfg(test)]
mod tests {
    #[test]
    fn test_accumulator() {
        let mut v = vec![4, 4, 4, 4];
        v.reverse();

        let mut v_dash = v
            .iter()
            .filter(|&n| *n != 0)
            .map(|&n| n)
            .collect::<Vec<i32>>();

        let mut new_vec = Vec::new();
        while !v_dash.is_empty() {
            let last = v_dash.pop();
            let second_last = v_dash.pop();

            if last.is_some() && second_last.is_some() {
                let last = last.unwrap();
                let second_last = second_last.unwrap();

                if last == second_last {
                    new_vec.push(last + second_last);
                } else {
                    new_vec.push(last);
                    v_dash.push(second_last);
                }
            } else if last.is_some() && second_last.is_none() {
                let last = last.unwrap();
                new_vec.push(last);
            }
        }

        println!("{:?}", new_vec)
    }
}
