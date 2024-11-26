use std::vec;
use std::fmt::Error;

pub fn bubblesort<T:Ord>(arr: &mut Vec<T>) -> (){ //just bubblesort
    for i in 1..arr.len()-1{
        for j in (i..arr.len()).rev(){
            if arr[j] < arr[j-1]{
                arr.swap(j, j-1);
            }
        }
    }

}