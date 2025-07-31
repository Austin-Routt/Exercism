//The enum
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

//The function that compares lists for comparison type
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let v = _first_list.clone();
    let u = _second_list.clone();
    //Check if equal
    if v.len() == u.len() && check(v, u) {
        return Comparison::Equal;
    }
    //Check if sublist
    else if v.len() < u.len() && check(v, u) {
        return Comparison::Sublist;
    }
    //Check if superlist
    else if v.len() > u.len() && check(u, v) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

//The comparison function
fn check<T: PartialEq>(x: &[T], y: &[T]) -> bool {
    if x == y {
        return true;
    }
    let mut b = y.clone();
    while !b.is_empty() {
        //compare x to b
        if b.starts_with(x) {
            return true;
        }
        //remove the first element of b
        b = &b[1..];
    }

    false
}
