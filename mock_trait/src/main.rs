#![feature(unboxed_closures, get_type_id)]

mod mock_trait;
mod reflection;

use mock_trait::MockTrait;
use std::collections::btree_set::BTreeSet;
use std::collections::BTreeMap;
use std::any::{Any, TypeId};
use std::cell::RefCell;

macro_rules! print_fn {
    () => {{
        fn fake_and_gay() {}
        println!("{}", get_type_name(&fake_and_gay))
    }}
}

thread_local!{
    static MOCKS: RefCell<BTreeMap<TypeId, String>> = RefCell::new(BTreeMap::new())
}

fn x<T>(_: T) {
    println!("x {:?}", x::<T>.get_mock_id());
}

fn main() {
    reflection::test_e();
}

struct AA<T>(T);

trait BB<U> {
    fn bb(&self, u: U) {
//        println!("{:?}", (&(&self as &BB<U>).bb).get_mock_id());
    }
}

impl<T, U> BB<U> for AA<T>{}


//fn get_type_name<T>(_: &T) -> &'static str {
//    unsafe{ type_name::<T>() }
//}

//fn old_main() {
//    MOCKS.with(|m| (*m.borrow_mut()).insert(main.get_mock_id(), "main".to_string()));
//    main.a();
//    main.b();
//    X::x.a();
//    X::x.b();
//    X::xx.a();
//    X::xx.b();
//    Y::<u32>::y.a();
//    Y::<u32>::y.b();
//    Y::<u32>::yy::<i64>.a();
//    Y::<u32>::yy::<i64>.b();
//    let f = |x: i32| x*2;
//    f.a();
//    f.b();
//    BTreeSet::<u32>::iter.a();
//    BTreeSet::<u32>::iter.b();
//    x::<u32>.a();
//    x::<u64>.a();
//    let s: u32 = 1;
//    let k: u32 = 2;
//    dss.a();
//    dss.b();
//    dss(&s, &k);
//}
//
//fn dss<'a, 'b>(i: &'a u32, j: &'b u32) {
//    x::<&'a u32>.a();
//    x::<&'b u32>.a();
//    x.c((i,));
//}


struct X;

impl X {
    fn x(&self) {
        println!("x");
    }

    fn xx() {
        println!("xx");
    }
}

struct Y<T> (T);

impl<T> Y<T> {
    fn y(&mut self, t: T) {
        println!("y");
        self.0 = t;
    }

    fn yy<U>(&mut self, t: T, u: U) -> U {
        println!("yy");
        self.0 = t;
        u
    }
}
