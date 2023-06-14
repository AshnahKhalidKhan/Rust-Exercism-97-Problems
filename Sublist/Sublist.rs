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
    
    let mut pointerInA: usize = 0;
    let mut pointerInB: usize = 0;
    let mut Answer: Comparison = Comparison::Unequal;
    
    //If A is longer than B, then A is either a superlist or they are unequal.
    if A.len() > B.len()
    {
        //While we have not found all characters in B, or we have not traversed A completely.
        while pointerInB < B.len() && pointerInA < A.len()
        {
            //If the characters at the current pointers in B and A match, check the next character in the pattern B.
            if B[pointerInB] == A[pointerInA]
            {
                pointerInB = pointerInB + 1;
            }
            else
            {
                //Anchor yourself on the current character in A. Backtrack to the last same character in B. If none match, start all over.
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
    
        //If all the characters in B were not traversed, they did not match with a set of consecutive characters in A; they are unequal.
        if pointerInB != B.len()
        {
            Answer = Comparison::Unequal;
        }
        //If B was traversed completely, and we already know that A and B do not have same lengths, then A is a superlist of B.
        else
        {
            Answer = Comparison::Superlist;
        }
    }
    //If A is shorter or as long as B, then A is either a sublist of B, equal to B, or they are unequal.
    else if A.len() <= B.len()
    {
        //While we have not found all characters in A, or we have not traversed B completely.
        while pointerInA < A.len() && pointerInB < B.len()
        {
            //If the characters at the current pointers in A and B match, check the next character in the pattern A.
            if A[pointerInA] == B[pointerInB]
            {
                pointerInA = pointerInA + 1;
            }
            else
            {
                //Anchor yourself on the current character in B. Backtrack to the last same character in A. If none match, start all over.
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
    
        //If A nd B have the same length and all the characters in A were traversed, they are equal.
        if A.len() == B.len() && pointerInA == pointerInB
        {
            Answer = Comparison::Equal;
        }
        //If all the characters in A were not traversed, they did not match with a set of consecutive characters in B; they are unequal.
        else if pointerInA != A.len()
        {
            Answer = Comparison::Unequal;
        }
        //If A was traversed completely, and we already know that A and B do not have same lengths, then A is a sublist of B.
        else
        {
            Answer = Comparison::Sublist;
        }
    }

    Answer
}
