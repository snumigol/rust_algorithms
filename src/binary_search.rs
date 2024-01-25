pub fn binary_search(haystack: Vec<i32>, needle: i32) -> bool {
    
    let mut low = 0;
    let mut high = haystack.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if haystack[mid] == needle {
            return true
        } else if haystack[mid] < needle {
            low = mid + 1;
        } else {
            high = mid;
        }

    }
    false
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn ex1(){
        assert_eq!(binary_search(vec![1,2,3,4,5,6,7,8,9,10], 10), true);
        }
    
    }
