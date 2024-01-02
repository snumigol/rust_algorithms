use std::cmp;

pub fn crystal_balls(breaks: Vec<bool>) -> isize {

    let jmp_amount = (breaks.len() as f64).sqrt().floor() as usize;

    let mut i = jmp_amount;
    while i < breaks.len() {
        if breaks[i]{
            break;
        }
        i += jmp_amount;
    }

    i = cmp::max(i as usize - jmp_amount as usize, 0);

    let mut j = 0;

    while j <= jmp_amount && i < breaks.len() {
        if breaks[i] {
            return i as isize;
        }
        j += 1;
        i += 1;
    }
    
    -1

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(crystal_balls(vec![false, false, true, true]), 2); 
    }

    #[test]
    fn ex2() {
        assert_eq!(crystal_balls(vec![false, false]), -1);
    }


}



