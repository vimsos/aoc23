pub trait Solver<const N: usize> {
    fn solve(&self);
}

impl<const N: usize> Solver<N> for (&str, [fn(&str) -> String; N]) {
    fn solve(&self) {
        let path = format!("src/input/{0}", self.0);

        let input = std::fs::read_to_string(&path)
            .expect(&format!("input file must be present at {}", &path));

        for (i, solve_fn) in self.1.iter().enumerate() {
            println!("part {}: {}", i + 1, solve_fn(&input))
        }
    }
}
