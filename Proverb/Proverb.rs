pub fn build_proverb(list: &[&str]) -> String {
    //unimplemented!("build a proverb from this list of items: {list:?}")
    let mut rhyme: String = String::new();
    if list.is_empty() == false
    {
        for word in 0..list.len() - 1
        {
            rhyme = rhyme + "For want of a ";
            rhyme = rhyme + list[word];
            rhyme = rhyme + " the ";
            rhyme = rhyme + list[word + 1];
            rhyme = rhyme + " was lost.\n";
        }
        rhyme = rhyme + "And all for the want of a ";
        rhyme = rhyme + list[0];
        rhyme = rhyme + ".";
    }
    rhyme
}
