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
    let mut Answer: Comparison = Comparison::Unequal;

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
        Answer = Comparison::Unequal;
    }
    else if pointerInA == pointerInB
    {
        Answer = Comparison::Equal;
    }
    else if pointerInA == _first_list.len() - 1 && _first_list.len() < _second_list.len()
    {
        Answer = Comparison::Sublist;
    }

    pointerInA = 0;
    pointerInB = 0;
    while pointerInB <= pointerInA && pointerInA < _first_list.len() && pointerInB < _second_list.len()
    {
        if _second_list[pointerInB] == _first_list[pointerInA]
        {
            pointerInB = pointerInB + 1;
        }
        else
        {
            pointerInB = 0;
        }
        pointerInA = pointerInA + 1;
    }

    if pointerInB == _second_list.len() - 1 && _second_list.len() < _first_list.len()
    {
        Answer = Comparison::Superlist;
    }

    Answer
}
