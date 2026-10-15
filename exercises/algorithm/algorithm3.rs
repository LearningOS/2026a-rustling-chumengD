/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE


fn sort<T>(array: &mut [T])
    where T:PartialOrd+Clone
{
    if let Some(_) =array.get(0){
        let mut index =0usize;
        
        while index <array.len(){
            let mut in_index= index;
            let mut min_index =index;

            while in_index<array.len(){
                if array.get(in_index).unwrap()<array.get(min_index).unwrap(){
                    min_index =in_index;
                }
                in_index+=1;
            }

            if min_index!=index {
                let temp =array.get(index).unwrap().clone();
                array[index] =array[min_index].clone();
                array[min_index] =temp;
            }

            index+=1;
        } 

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}