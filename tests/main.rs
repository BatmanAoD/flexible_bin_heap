use std::collections::BinaryHeap;

/*
#[test]
fn ord_by_cmp() {
    let heap = BinaryHeap<i32>::with_cmp(|a,b| a < b);
    heap.push(5);
    heap.push(8);
    heap.push(1);
    heap.push(2);
    heap.push(9);
    heap.push(0);
    // println!("{:#?}", heap);     // TODO - resolve `fmt` issue
}
*/

use flexible_bin_heap::with_comparator;
use std::cmp::Ordering;

#[derive(Debug)]
struct MyData {
    data1: i32,
    data2: f64
}

#[with_comparator(data2)]
#[derive(Debug)]
struct MyDataByData2(MyData);

#[test]
fn order_using_macro() {
    let heap = BinaryHeap::<MyDataByData2>::new();
    /* XXX - need to transform each `MyData` into `MyDataByData2`
    heap.push(MyData { data1: 5, data2: 3.2 });
    heap.push(MyData { data1: 8, data2: 0.7 });
    heap.push(MyData { data1: 1, data2: 345.2 });
    heap.push(MyData { data1: 2, data2: 11.1 });
    heap.push(MyData { data1: 9, data2: -34.3 });
    heap.push(MyData { data1: 0, data2: -4.0 });
    */
    println!("{:#?}", heap);
}
