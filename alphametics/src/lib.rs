use std::collections::{HashMap};

fn combinations<T, X>(k: usize, arr: &[&T], f: &impl Fn(&[&T]) -> Option<X>) -> Option<X> {
    fn comb0<T, X>(k: usize, dic: &[&T], buf: &[&T], f: &impl Fn(&[&T]) -> Option<X>) -> Option<X> {
        if k == 0 {
            return f(buf);
        }
        for i in 0..=dic.len() - k {
            let mut vec = buf.to_vec();
            vec.push(&dic[i]);
            let ret = comb0(k - 1, &dic[i + 1..dic.len()], &vec, f);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }
    if k == arr.len() {
        return f(arr);
    }
    comb0(k, arr, &Vec::default(), f)
}


fn perm_heap<T, X>(arr: &mut [T], f: &impl Fn(&[T]) -> Option<X>) -> Option<X>
{
    fn heap0<T, X>(k: usize, arr: &mut [T], f: &impl Fn(&[T]) -> Option<X>) -> Option<X> {
        if k == 1 {
            f(arr)
        } else {
            let ret = heap0(k - 1, arr, f);
            if ret.is_some() {
                return ret;
            }
            for i in 0..k - 1 {
                if k % 2 == 0 {
                    arr.swap(i, k - 1);
                } else {
                    arr.swap(0, k - 1);
                }
                let ret = heap0(k - 1, arr, f);
                if ret.is_some() {
                    return ret;
                }
            }
            None
        }
    }
    heap0(arr.len(), arr, f)
}

fn permutations_between<T: Clone, X>(k: usize, arr: &[&T], f: &impl Fn(&[&T]) -> Option<X>) -> Option<X> {
    combinations(k, arr, &|x| perm_heap(&mut x.to_vec(), f))
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let vec: Vec<&str> = input.split("==").collect();
    if vec.len() != 2 {
        return None;
    }
    let res = vec[1].trim();
    let adds: Vec<&str> = vec[0].split('+').map(|s| s.trim()).collect();

    let coeffs: HashMap<char, i64> = adds.iter().flat_map(|add| add.chars().rev().enumerate())
        .map(|(i, c)| (c, 10i64.pow(i as u32)))
        .chain(res.chars().rev().enumerate().map(|(i, c)| (c, -(10i64.pow(i as u32)))))
        .fold(HashMap::new(), |mut acc, (c, coeff)| {
            *acc.entry(c).or_default() += coeff;
            acc
        });

    let forbidden: HashMap<char, i64> = adds.iter().chain(std::iter::once(&res))
        .map(|s| (s.chars().next().unwrap(), 0))
        .collect();


    permutations_between(coeffs.len(), &[&0, &1, &2, &3, &4, &5, &6, &7, &8, &9], &|j| {
        coeffs.iter().enumerate()
            .map(|(curr_index, (char, coeff))| (char, j[curr_index], coeff))
            .try_fold((0i64, Vec::new()), |(sum, mut chars), (c, num, coeff)| {
                if forbidden.get(&c).map_or(false, |f| f == num) {
                    None
                } else {
                    chars.push((c, num));
                    Some((sum + coeff * num, chars))
                }
            })
            .filter(|(total, _)| total == &0)
            .map(|(_, vec)| {
                vec.iter().fold(HashMap::new(), |mut acc, (char, value)| {
                    acc.insert(**char, **value as u8);
                    acc
                })
            })
    })
}
