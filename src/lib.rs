use std::collections::BinaryHeap;

use std::cmp::Ordering;
use std::iter::FromIterator;
use std::fmt::Debug;

// XXX how to get `Debug` here? Need to mark it for `T` everywhere?
// XXX similar: Clone
trait ContainerLike<T: Ord>: /* Clone + Debug + */ Default + FromIterator<T> + IntoIterator { }

trait DynCmpContainer<T: Ord, C: ContainerLike<T>>: ContainerLike<T> {
    // XXX associated traits, or trait aliases?
    // trait Comparator: Fn(&T, &T) -> Ordering {}

    fn set_cmp(comparator: impl Fn(&T, &T) -> Ordering);
}

trait WithDynCmp<T: Ord, C: ContainerLike<T>, D: DynCmpContainer<T, C>> {
    fn with_cmp(comparator: impl Fn(&T, &T) -> Ordering) -> C;
}

// ---

impl<T: Ord> ContainerLike<T> for BinaryHeap<T> { }

#[derive(Debug)]
struct BinaryHeapDynCmp<T: Ord> {
    container: BinaryHeap<T>,
    cmp: Box<dyn Fn(&T, &T) -> Ordering>
}

impl<T: Ord> ContainerLike<T> for BinaryHeap<T> { }     // XXX - forward everything to `container`

impl<T: Ord> DynCmpContainer<T, BinaryHeap<T>> for BinaryHeapDynCmp<T> {
    fn set_cmp(comparator: impl Fn(&T, &T) -> Ordering) {
        unimplemented!()
    }
}

impl<T: Ord> WithDynCmp<T, BinaryHeap<T>, BinaryHeapDynCmp<T>> for BinaryHeap<T> {
    fn with_cmp(comparator: impl Fn(&T, &T) -> Ordering) -> BinaryHeapDynCmp<T> {
        BinaryHeapDynCmp::<T> {
            container: BinaryHeap::new(),
            cmp: Box::new(comparator)
        }
    }
}
