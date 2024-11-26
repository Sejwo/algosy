use std::vec;
use std::fmt::Error;
use std::time::{Instant, Duration};

pub fn insert_sort_isize(arr: Vec<isize>) -> Result<Vec<isize>, Error> { //trying to move from isize to signed integers
    let n = arr.len();
    let mut arr = arr.clone();

    if n == 0 {
        return Ok(arr);
    }
    
    for i in 1..n {
        let key = arr[i];
        let mut j:usize = i; 

        while j > 0 && arr[j-1] > key {
            arr[j] = arr[j-1];
            j -= 1;
        }
        arr[j] = key; 
    }
    Ok(arr)
}

pub fn insert_sort_i(arr: Vec<i64>) -> Result<Vec<i64>, Error> { //i64 based solution
    let n = arr.len();
    let mut arr = arr.clone();

    if n == 0 {
        return Ok(vec![]);
    }
    
    for i in 1..n {
        let key = arr[i];
        let mut j:usize = i; 

        while j > 0 && arr[j -1]> key {
            arr[j] = arr[j-1];
            j -= 1;
        }
        
        arr[j] = key; 
    }
    Ok(arr)
}
pub fn insert_sort_generic_type<T:Ord>(arr:Vec<T>) -> Result<Vec<T>, Error> where T:Clone, T:Copy{ // i like the performance here the best, also it's the most versitile as it should accept any nuemrical vector and output it in the same format TODO: Check other than i64
    let mut arr = arr.clone();
    if arr.len() == 0 {
        return Ok(vec![]);
    }
    
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j:usize = i; 
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j-1];
            j -= 1;
        }
        arr[j] = key; 
    }
    Ok(arr)
}

pub fn insert_sort_mod_gt<T:Ord>(arr: &mut Vec<T>) where T:Copy{ //the fastest solution I've found, leverages a mutable borrow to rearange without cloning(copying is necessary for line 73, maybe arr.swap() could be used here)
    if arr.len() == 0{
        return;
    }
    for i in 1..arr.len(){
        let key = arr[i];
        let mut j:usize = i;
        while j>0 && arr[j-1] > key{
            arr[j] = arr[j-1];
            j-=1;
        }
        arr[j] = key;
    }
}

