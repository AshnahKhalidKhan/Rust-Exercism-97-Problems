#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");

    // let mut AllAInB: bool = true;
    // let mut AllBInA: bool = true;

    let mut pointerInA: usize = 0;
    let mut pointerInB: usize = 0;

    while pointerInA <= pointerInB && pointerInA < _first_list.len() && pointerInB < _second_list.len()
    {
        if _first_list[pointerInA] == _second_list[pointerInB]
        {
            pointerInA = pointerInA + 1;
        }
        else
        {
            pointerInA = 0;
        }
        pointerInB = pointerInB + 1;
    }

    if pointerInA != _first_list.len() - 1 && pointerInB == _second_list.len() - 1
    {
        return Comparison::Unequal;
    }
    else if pointerInA == pointerInB
    {
        return Comparison::Equal;
    }
    else if pointerInA == _first_list.len() - 1 && _first_list.len() < _second_list.len()
    {
        return Comparison::Sublist;
    }
    else
    {
        return Comparison::Superlist;
    }
}
