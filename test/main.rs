use crate::BinaryHeap;

#[test]
fn ord_by_cmp() {
    let heap = BinaryHeap::with_cmp(|a,b| a < b);
    heap.push(5);
    heap.push(8);
    heap.push(1);
    heap.push(2);
    heap.push(9);
    heap.push(0);
    println!("{:#?}", heap);
}
