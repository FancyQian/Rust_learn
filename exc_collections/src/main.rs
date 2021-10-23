// 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
// 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
// 所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
// 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。

use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    println!("=========================vector's average code=========================");
    let v = vec!(1, 2, 2, 3, 4, 5, 6, 6 ,6, 7, 8, 9, 10);
    let average = my_vector_get_average(&v);

    println!("vector's average value is {:?}", average);

    println!("=========================Median's average code=========================");
    let mut v  = vec!(123, 2, 2, 322, 4, 5, 64, 1 ,61111, 7, 8, 3419, 10);
    let med = my_vector_get_median(&mut v);
    println!("vector is {:?}", v);
    println!("vector Median is {:?}", med);

    println!("=========================众数 code=========================");
    let v  = vec!(123, 123, 2, 322, 4, 5, 64, 1 ,64, 7, 8, 3419, 10, 2, 123);
    let mut vmap = HashMap::new();

    for val in v {
        let count = vmap.entry(val).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut max_vec_val = 0;
    for (key, value) in &vmap {
        if max < *value {
            max_vec_val = *key;
            max = *value; 
        }
    }
    println!("=========================众数 是 {}=========================", max_vec_val);

}

fn my_vector_get_average(v: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in v {
        sum += i;
    }
    let v_len = v.len() as i32;
    sum/v_len
}

fn my_vector_get_median(v: &mut Vec<i32>) -> i32 {
    v.sort();

    if v.len() % 2 == 0 { 
        let mid_index = v.len() / 2;
        (v[mid_index] + v[mid_index+1]) / 2
    } else {
        let mid_index = (v.len() + 1) /2;
        v[mid_index]
    }
}