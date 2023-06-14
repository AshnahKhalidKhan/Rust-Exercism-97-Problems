#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(A: &[T], B: &[T]) -> Comparison
{
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");

    // let mut AllAInB: bool = true;
    // let mut AllBInA: bool = true;

    if A.len() == 0 && B.len() == 0
    {
        return Comparison::Equal;
    }
    else if A.len() == 0 && B.len() != 0
    {
        return Comparison::Sublist;
    }
    else if A.len() != 0 && B.len() == 0
    {
        return Comparison::Superlist;
    }


    let mut pointerInA: usize = 0;
    let mut pointerInB: usize = 0;
    let mut Answer: Comparison = Comparison::Unequal;
    
    if A.len() > B.len()
    {
        while pointerInA < A.len() && pointerInB < B.len()
        {
            if B[pointerInB] == A[pointerInA]
            {
                pointerInB = pointerInB + 1;
            }
            else
            {
                while pointerInB > 0
                {
                    pointerInB = pointerInB - 1;
                    if B[pointerInB] == A[pointerInA]
                    {
                        pointerInB = pointerInB + 1;
                        break;
                    }
                }
            }
            pointerInA = pointerInA + 1;
        }
    
        if pointerInB != B.len() //&& pointerInA == A.len()
        {
            Answer = Comparison::Unequal;
        }
        else //if pointerInA == A.len() - 1 && A.len() < B.len()
        {
            Answer = Comparison::Superlist;
        }
    }
    else if A.len() <= B.len()
    {
        while pointerInA < A.len() && pointerInB < B.len()
        {
            if A[pointerInA] == B[pointerInB]
            {
                pointerInA = pointerInA + 1;
            }
            else
            {
                while pointerInA > 0
                {
                    pointerInA = pointerInA - 1;
                    if A[pointerInA] == B[pointerInB] 
                    {
                        pointerInA = pointerInA + 1;
                        break;
                    }
                }
            }
            pointerInB = pointerInB + 1;
        }
    
        if pointerInA != A.len() && pointerInB == B.len()
        {
            Answer = Comparison::Unequal;
        }
        else if pointerInA == pointerInB && A.len() == B.len()
        {
            Answer = Comparison::Equal;
        }
        else //if pointerInA == A.len() - 1 && A.len() < B.len()
        {
            Answer = Comparison::Sublist;
        }
    }

    Answer
}
