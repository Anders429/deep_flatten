#[cfg(test)]
mod tests {
    #[test]
    fn deep_flatten() {
        assert_eq!(vec![vec![1], vec![2, 3], vec![], vec![4, 5, 6]].iter().deep_flatten().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6]);
    }
}
