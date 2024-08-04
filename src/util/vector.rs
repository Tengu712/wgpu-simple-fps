pub fn get_upto<'a, T>(v: &'a Vec<T>, max_size: usize) -> &'a [T] {
    if v.len() <= max_size {
        v.as_slice()
    } else {
        &v.as_slice()[0..max_size]
    }
}

pub fn get_consecutive_somes<T, U, F>(v: &[Option<T>], f: F) -> Vec<(usize, Vec<U>)>
where
    U: Clone,
    F: Fn(&T) -> U,
{
    let mut group = Vec::new();
    let mut cv = Vec::new();
    let mut ci = 0;
    for (i, n) in v.into_iter().enumerate() {
        if let Some(n) = n {
            if cv.is_empty() {
                ci = i;
            }
            cv.push(f(n));
        } else if !cv.is_empty() {
            group.push((ci, cv.clone()));
            cv.clear();
        }
    }
    if !cv.is_empty() {
        group.push((ci, cv.clone()));
    }
    group
}
