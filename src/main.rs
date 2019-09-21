extern crate rand;

use rand::{Rng, random};
use std::time;
use std::time::{UNIX_EPOCH, SystemTime};
use std::i32;

fn bin_search(v: &Vec<i32>, elem: i32) -> Option<usize> {
    let mut start: usize = 0;
    let mut end = v.len();

    while start != end {
        let mid = start / 2 + end / 2;
        if elem == v[mid] {
            return Some(mid)
        }
        else if elem < v[mid] {
            end = mid;
        }
        else {
            start = mid + 1;
        }
    }
    None
}

fn bin_search_rec(v: &Vec<i32>, elem: i32, start: usize, end: usize) -> Option<usize> {
    if start == end {
        return None
    }
    let mid = start / 2 + end / 2;
    //println!("mid = {}, v[mid] = {}, start = {}, end = {}", mid, v[mid], start, end);
    if elem == v[mid] {
        Some(mid)
    }
    else if elem < v[mid] {
        bin_search_rec(v, elem, start, mid)
    }
    else {
        bin_search_rec(v, elem, mid + 1, end)
    }
}

fn merge(array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32> {
    let n = array1.len() + array2.len();
    let mut res_arr: Vec<i32> = Vec::with_capacity(n);
    let mut i1 = 0;
    let mut i2 = 0;
    for _ in 0 .. n {
        if i1 < array1.len() && i2 < array2.len() {
            if array1[i1] > array2[i2] {
                res_arr.push(array2[i2]);
                i2 += 1;
            } else {
                res_arr.push(array1[i1]);
                i1 += 1;
            }
        }
        else {
            if i1 >= array1.len() && i2 >= array2.len() {
                break;
            }
            else if i1 >= array1.len() {
                res_arr.push(array2[i2]);
                i2 += 1;
            }
            else {
                res_arr.push(array1[i1]);
                i1 += 1;
            }
        }
    }
    res_arr
}

fn dual_pivot_qs(v: Vec<i32>) -> Vec<i32> {
    //println!("{:?}", v);
    if v.len() > 2 {
        let mut random = rand::thread_rng();
        //println!("start = {}, end = {}", 0, v.len() - 1);
        let split1 = random.gen_range(0, v.len() - 2);
        //println!("split1 = {}", split1);
        let split2 = random.gen_range(split1 + 1, v.len() - 1);
        //println!("split2 = {}\n", split2);
        let part1 = dual_pivot_qs(v.iter().cloned().take(split1).collect());
        let part2 = dual_pivot_qs(v.iter().cloned().take(split2).skip(split1).collect());
        let part3 = dual_pivot_qs(v.iter().cloned().skip(split2).collect());
        merge(merge(part1, part2), part3)
    }
    else if v.len() == 2{
        if v[0] > v[1] {
            vec![v[1], v[0]]
        }
        else {
            v
        }
    }
    else {
        v
    }
}

fn main() {
    //EX1:
    /*let mut random = rand::thread_rng();
    //Array creation
    let start: i32 = 1;
    let end: i32 = 1000000000;
    let v: Vec<i32> = (start .. end).collect();
    let elem = v[random.gen_range(0, v.len() - 1)];

    //Time measuring
    let mut avg = 0;
    let mut avg_rec = 0;
    let iters = 30;
    for _ in 0..iters {
        let t_start = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Error").as_nanos();
        let res_rec = bin_search_rec(&v, elem, 0, v.len());
        let t_end = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Error").as_nanos();
        println!("t_end = {}, t_start = {}", t_end, t_start);
        avg_rec += t_end - t_start;

        let t_start = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Error").as_nanos();
        let res = bin_search(&v, elem);
        let t_end = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Error").as_nanos();
        avg += t_end - t_start;
        println!("Place of {} = {:?} (rec)", elem, res_rec);
        println!("Place of {} = {:?}", elem, res);
        assert_eq!(res, res_rec);
        match res_rec {
            Some(i) => assert_eq!(elem, v[i]),
            None => assert!(match v.binary_search(&elem) {Ok(_) => false, Err(_) => true})
        }
    }
    avg = avg / iters;
    avg_rec = avg_rec / iters;
    println!("rec iter");
    println!("{} {}", avg_rec, avg);*/

    //EX5:
    let start_size = 10000;
    let end_size = 150000;
    let step = 10000;
    let iters = 30;

    let mut results: Vec<u128> = Vec::with_capacity(end_size / start_size);
    let mut random = rand::thread_rng();

    for size in (start_size ..= end_size).step_by(step) {
        let mut arr: Vec<i32> = Vec::with_capacity(size);
        for _ in 0 .. size {
            arr.push(random.gen_range(0, i32::MAX))
        }

        let mut time_acc = 0;
        for _ in 0..iters {
            // let arr_copy = arr.clone();
            let start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH).expect("Error");
            let sorted_arr = dual_pivot_qs(arr.clone());
            //arr.sort();
            let end_time = SystemTime::now()
                .duration_since(UNIX_EPOCH).expect("Error");
            let whole_time = end_time.as_millis() - start_time.as_millis();
            time_acc += whole_time;
            //arr = arr_copy;
        }

        results.push(time_acc / iters);
    }
    for i in results {
        println!("{}", i);
    }
}
