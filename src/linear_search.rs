pub fn linear_search(haystack: &[i32], needle: i32) -> bool{

    for &item in haystack {
        if item == needle {
            return true;
            }
    } 
    false

}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(linear_search(&[1,2,3,4], 3), true);
        }
    #[test]
    fn ex2() {
        assert_eq!(linear_search(&[1,2,3,4], 4), false);
        }



}
