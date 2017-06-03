#![feature(unboxed_closures, get_type_id)]

use std::collections::btree_set::BTreeSet;
use std::collections::BTreeMap;
use std::any::{Any, TypeId};
use std::cell::RefCell;
mod reflection;

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
    println!("x {:?}", x::<T>.get_paint_id());
}

fn main() {
    reflection::test_e();
}

struct AA<T>(T);

trait BB<U> {
    fn bb(&self, u: U) {
//        println!("{:?}", (&(&self as &BB<U>).bb).get_paint_id());
    }
}

impl<T, U> BB<U> for AA<T>{}


//fn get_type_name<T>(_: &T) -> &'static str {
//    unsafe{ type_name::<T>() }
//}

fn old_main() {
    MOCKS.with(|m| (*m.borrow_mut()).insert(main.get_paint_id(), "main".to_string()));
    main.a();
    main.b();
    X::x.a();
    X::x.b();
    X::xx.a();
    X::xx.b();
    Y::<u32>::y.a();
    Y::<u32>::y.b();
    Y::<u32>::yy::<i64>.a();
    Y::<u32>::yy::<i64>.b();
    let f = |x: i32| x*2;
    f.a();
    f.b();
    BTreeSet::<u32>::iter.a();
    BTreeSet::<u32>::iter.b();
    x::<u32>.a();
    x::<u64>.a();
    let s: u32 = 1;
    let k: u32 = 2;
    dss.a();
    dss.b();
    dss(&s, &k);
}

fn dss<'a, 'b>(i: &'a u32, j: &'b u32) {
    x::<&'a u32>.a();
    x::<&'b u32>.a();
    x.c((i,));
}


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

trait MockTrait<T, O> {
    fn a(&self);
    fn b(&self);
    fn get_paint_id(&self) -> TypeId;
    fn c(&self, T) -> MockResult<T, O>;
}

enum MockResult<T, O> {
    Continue(T),
    Return(O),
}

//impl<T, O, F: Fn(T) -> O> MockTrait<T, O> for F {
impl<T, O, F: FnOnce<T, Output=O>> MockTrait<T, O> for F {
    fn a(&self) {
        println!("{:?}", self.get_paint_id());
    }

    fn get_paint_id(&self) -> TypeId {
        (||()).get_type_id()
    }

    fn b(&self) {
        println!("{:?} AAA", self.get_paint_id());
    }

    fn c(&self, args: T) -> MockResult<T, O> {
        MockResult::Continue(args)
    }
}
