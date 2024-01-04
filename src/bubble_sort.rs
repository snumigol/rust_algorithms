pub fn bubble_sort(arr: &mut Vec<i32>) {
    
    for i in 0..arr.len(){
        for j in 0..arr.len() - 1 - i {
                 
            if arr[j] > arr [j + 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
                }
        }
            
    }
    
}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn ex1() {
        let mut arr = vec![5, 2, 9, 1, 5, 6];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 5, 6, 9]);
        }
    
}
