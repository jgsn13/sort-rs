pub fn bubble_sort(v: &mut Vec<i32>) {
    let n = v.len();
    for _ in 0..n {
        for i in 0..(n - 1) {
            if v[i] > v[i + 1] {
                let aux = v[i];
                v[i] = v[i + 1];
                v[i + 1] = aux;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_correctly() {
        let mut v: Vec<i32> = vec![10, 2, -1, 3, 9, 7, 18, -13];
        bubble_sort(&mut v);
        assert_eq!(v, vec![-13, -1, 2, 3, 7, 9, 10, 18]);
    }
}
