use std::vec;
use std::fmt::Error;

 
fn merge<T:Ord + std::fmt::Debug + std::clone::Clone>(arr: &mut [T], left: &[T], right: &[T]){
    let mut left_index: usize = 0;
    let mut right_index:usize = 0;
    let mut arr_index:usize = 0;

    while left_index < left.len() && right_index < right.len(){ //comparing split array until reaching the last element
        if left[left_index] <= right[right_index]{
            arr[arr_index] = left[left_index].clone(); //may not be the most optimal, but the assumption is that I work on slices so popping will not work here
            left_index +=1;
        }else{ 
            arr[arr_index] = right[right_index].clone();
            right_index += 1;
        
        } 
    arr_index +=1;
    }
    while left_index < left.len(){ // after breaking out of the previous loop I need to 'return' one array i'll be working on so all of the unused elements are appened at the end
        arr[arr_index] = left[left_index].clone();
        left_index +=1;
        arr_index +=1;
    }
    while right_index < right.len(){
        arr[arr_index] = right[right_index].clone();
        right_index +=1;
        arr_index +=1;
    }
    //println!("arr {:?} \n", arr);
}

pub fn merge_sort<T:Ord + std::fmt::Debug + std::clone::Clone>(arr:&mut Vec<T>){
    if arr.len() <=1{
        return;
    }
    let mid = arr.len()/2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(arr,&left,&right);

}

