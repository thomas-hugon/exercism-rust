#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> Comparison {
    // match (list1.len(), list2.len()) {
    //     (0, 0) => Comparison::Equal,
    //     (0, _) => Comparison::Sublist,
    //     (_, 0) => Comparison::Superlist,
    //     (a, b) if a < b && is_contained(list1, list2) => Comparison::Sublist,
    //     (a, b) if a > b && is_contained(list2, list1) => Comparison::Superlist,
    //     _ if list1[0] == list2[0] && list1 == list2 => Comparison::Equal,
    //     _ => Comparison::Unequal,
    // }
    if list1.len() < list2.len() && is_contained(list1, list2) {
        Comparison::Sublist
    } else if list2.len() < list1.len() && is_contained(list2, list1) {
        Comparison::Superlist
    } else if list1 == list2 {
        Comparison::Equal
    } else {
        Comparison::Unequal
    }
}

#[inline]
fn is_contained<T: PartialEq>(contained: &[T], container: &[T]) -> bool {
    let contained_len = contained.len();
    if contained_len == 0 {
        return true;
    }

    container
        .windows(contained_len)
        .filter(|z| z[0] == contained[0])
        .any(|z| z == contained)
}
