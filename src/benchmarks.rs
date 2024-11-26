use crate::insert_sort::*;
use crate::merge_sort::*;
use crate::bubblesort::*;
use std::time::{Instant, Duration};

fn construct_unsorted_vectors(n:usize)-> Vec<i64> { 
    let vecs:Vec<Vec<i64>> =vec![
        vec![12, 45, 78, 23, 56, 89, 34, 67, 90, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 13,
         46, 79, 24, 57, 80, 35, 68, 91, 12, 23, 34, 45, 56, 67, 78, 89, 99, 100, 11, 21, 31, 41, 51, 61, 71, 81, 91, 92,
         93, 94, 95, 96, 97, 98, 21, 32, 43, 54, 65, 76, 87, 98, 19, 30, 41, 52, 63, 74, 85, 96, 18, 29, 40, 51, 62, 73,
         84, 95, 17, 28],
        vec![102, 205, 308, 401, 504, 607, 710, 813, 916, 19, 222, 325, 428, 531, 634, 737, 840, 943, 46, 149, 252, 355, 458,
         561, 664, 767, 870, 973, 76, 179, 282, 385, 488, 591, 694, 797, 900, 103, 206, 309, 412, 515, 618, 721, 824, 927,
         30, 133, 236, 339, 442, 545, 648, 751, 854, 957, 60, 163, 266, 369, 472, 575, 678, 781, 884, 987, 90, 193, 296,
         399, 502, 605, 708, 811, 914, 17],
        vec![25, 47, 68, 12, 34, 57, 23, 69, 91, 15, 67, 29, 80, 22, 48, 39, 54, 77, 19, 88, 35, 66, 41, 82, 37, 96, 43, 78,
         92, 53, 24, 87, 63, 51, 90, 45, 74, 69, 81, 17, 40, 93, 50, 28, 62, 75, 64, 84, 38, 61, 46, 56, 49, 32, 60, 44,
         30, 89, 58, 71, 18, 65, 21, 52, 33, 42, 25, 59, 31, 48, 72, 26, 13, 27, 14, 36, 11, 55, 76, 83, 16, 20, 98, 79,
         99, 86, 70, 85],
        vec![301, 502, 103, 204, 405, 606, 707, 808, 909, 100, 201, 302, 403, 504, 605, 706, 807, 908, 109, 210, 311, 412, 513,
         614, 715, 816, 917, 18, 119, 220, 321, 422, 523, 624, 725, 826, 927, 28, 129, 230, 331, 432, 533, 634, 735, 836,
         937, 38, 139, 240, 341, 442, 543, 644, 745, 846, 947, 48, 149, 250, 351, 452, 553, 654, 755, 856, 957, 58, 159,
         260, 361, 462, 563, 664, 765, 866],
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52,
         54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 98, 100, 99, 97, 95, 93,
         91, 89, 87, 85, 83, 81, 79, 77, 75, 73, 71, 69, 67, 65, 63, 61, 59, 57, 55, 53, 51, 49, 47, 45, 43, 41, 39, 37,
         35, 33],
        vec![67, 12, 33, 94, 57, 18, 75, 36, 89, 24, 43, 78, 96, 81, 27, 54, 60, 45, 19, 11, 44, 91, 64, 88, 42, 34, 32, 48, 15,
         53, 22, 23, 39, 63, 41, 77, 58, 68, 25, 26, 62, 46, 93, 28, 29, 31, 30, 21, 17, 16, 59, 37, 61, 47, 52, 20, 51, 38,
         35, 40, 49, 50, 13, 56, 65, 14, 66, 73, 92, 74, 76, 72, 71, 95, 80, 90, 87, 79, 83, 85, 84, 82, 86, 73, 70, 55],
        vec![61,-1,15,16,20,19,56,4,3,2,1,5,10,21,37,1001,10002,115,314,2137,99999,98412,9471894,234,8357,1284,15978,56156,687,3,
        64145, 3187,6236,13616,35,753,87,981,163,7357,1049,85,98,57,19,09,851,159,7214,294,915,37856, 81375,912,16,925,8124,
        487,249,92537,23857,12984,195]
    ];
    return vecs[n].clone();
}
fn test_merge_sort(in_array: Vec<i64>) -> Duration{
    let mut in_array = in_array.clone();
    let now = Instant::now();
    merge_sort(&mut in_array);
    now.elapsed()
   
}
fn test_isize(in_array: Vec<i64>) -> Duration{
    let now = Instant::now();
    let _out_array:Vec<i64> = insert_sort_i(in_array.clone()).unwrap();
    now.elapsed()
}
fn test_i64(in_array: Vec<i64>) -> Duration{
    let now = Instant::now();
    let _out_array_i64:Vec<i64> = insert_sort_i(in_array.clone()).unwrap();
    now.elapsed()
}
fn test_generic_type(in_array: Vec<i64>) -> Duration{
    let now = Instant::now();
    let _out_array_generic:Vec<i64> = insert_sort_generic_type(in_array.clone()).unwrap();
    now.elapsed()
    
}

fn test_mod_gt(in_array: Vec<i64>) -> Duration{
    let mut in_array = in_array.clone();
    let now = Instant::now();
    insert_sort_mod_gt(&mut in_array);
    now.elapsed()

}
fn test_bubblesort(in_array: Vec<i64>)->Duration{
    let mut in_array = in_array.clone();
    let now = Instant::now();
    bubblesort(&mut in_array);
    now.elapsed()
}

pub fn compare_average_time(n:u32){ //so yeah, obviously execution time varies, and is not the key metric when comparing algorithms but all of the algorithms are 
    let mut i64_sum:u128 = 0;
    let mut isize_sum:u128 = 0; 
    let mut gen_type_sum:u128 = 0;
    let mut mod_gen_sum:u128 = 0;
    let mut merg_sum:u128 = 0;
    let mut bubblesort_sum:u128 = 0;
    let mut inner_counter:usize = 0;
    for _i in 0..n{
        let in_array = construct_unsorted_vectors(inner_counter);
        i64_sum += test_i64(in_array.clone()).as_nanos();
        isize_sum += test_isize(in_array.clone()).as_nanos();
        gen_type_sum += test_generic_type(in_array.clone()).as_nanos();
        mod_gen_sum += test_mod_gt(in_array.clone()).as_nanos();
        merg_sum += test_merge_sort(in_array.clone()).as_nanos();
        bubblesort_sum += test_bubblesort(in_array.clone()).as_nanos();
        if inner_counter ==6{
            inner_counter = 0;
        }
    }
    let i64_avg = i64_sum/(n as u128);
    let isize_avg = isize_sum/(n as u128);
    let gen_type_avg = gen_type_sum/(n as u128); 
    let mod_gen_avg = mod_gen_sum/(n as u128);
    let merg_sum_avg = merg_sum/(n as u128);
    let bubblesort_avg = bubblesort_sum/(n as u128);
    println!("
            wynik i64 {:?} \n 
            wynik isize {:?} \n 
            wynik generic types {:?} \n 
            wynik mut generic types {:?} \n
            wynik merge sort generic types {:?}\n
            wynik bubblesort {:?}",
            i64_avg, isize_avg, gen_type_avg, mod_gen_avg, merg_sum_avg, bubblesort_avg);
}