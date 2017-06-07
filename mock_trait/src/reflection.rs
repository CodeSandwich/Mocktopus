use MockTrait;

pub fn test() {
    println!("Standalone");
    standalone();
    println!("outer {:?}", standalone.get_paint_id());

    println!("\nGeneric standalone");
    standalone_generic(0i32);
    standalone_generic(0u32);
    println!("outer {:?}", standalone_generic::<u32>.get_paint_id());

    println!("\nGeneric standalone &'a u32");
    standalone_generic(&0i32);
    standalone_generic(&0u32);
    println!("outer {:?}", standalone_generic::<&u32>.get_paint_id());

    println!("\nStruct impl");
    Struct::implem();
    println!("outer {:?}", Struct::implem.get_paint_id());

    println!("\nStruct generic impl");
    Struct::generic_impl(0i32);
    Struct::generic_impl(0u32);
    println!("outer {:?}", Struct::generic_impl::<u32>.get_paint_id());

    println!("\nGeneric struct impl");
    GenericStruct::<i32>::implem();
    GenericStruct::<u32>::implem();
    println!("outer {:?}", GenericStruct::<u32>::implem.get_paint_id());

    println!("\nGeneric struct generic impl");
    GenericStruct::<i32>::generic_impl(0u64);
    GenericStruct::<u32>::generic_impl(0i64);
    GenericStruct::<u32>::generic_impl(0u64);
    println!("outer {:?}", GenericStruct::<u32>::generic_impl::<u64>.get_paint_id());

    println!("\nTrait impl for struct");
    println!("\nTrait generic impl for struct");
    println!("\nTrait generic impl for generic struct");
    println!("\nGeneric trait generic impl for generic struct");
}

fn standalone() {
    println!("inner {:?}", standalone.get_paint_id());
}

fn standalone_generic<T>(_: T) {
    println!("inner {:?}", standalone_generic::<T>.get_paint_id());
}

struct Struct;

impl Struct {
    fn implem() {
        println!("inner {:?}", Self::implem.get_paint_id());
    }

    fn generic_impl<T>(_: T) {
        println!("inner {:?}", Self::generic_impl::<T>.get_paint_id());
    }
}

struct GenericStruct<T>(T);

impl<T> GenericStruct<T> {
    fn implem() {
        println!("inner {:?}", Self::implem.get_paint_id());
    }

    fn generic_impl<U>(_: U) {
        println!("inner {:?}", Self::generic_impl::<U>.get_paint_id());
    }
}

trait Trait {
    fn func() {
        println!("inner {:?}", Self::func.get_paint_id());
    }

    fn generic_func<T>(_: T) {
        println!("inner {:?}", Self::generic_func::<T>.get_paint_id());
    }
}

struct Defaulter1;

impl Trait for Defaulter1 {}

struct Defaulter2;

impl Trait for Defaulter2{}

struct Overrider1;

impl Trait for Overrider1 {
    fn func() {
        println!("inner {:?}", Self::func.get_paint_id());
    }

    fn generic_func<T>(_: T) {
        println!("inner {:?}", Self::generic_func::<T>.get_paint_id());
    }
}

struct Overrider2;

impl Trait for Overrider2 {
    fn func() {
        println!("inner {:?}", Self::func.get_paint_id());
    }

    fn generic_func<T>(_: T) {
        println!("inner {:?}", Self::generic_func::<T>.get_paint_id());
    }
}

trait ETrait<T, U> {
    fn e_fn<V, W>(&self, _: T, _: U, _: V, _: W) {
        println!("inner {:?}", Self::e_fn::<V, W>.get_paint_id());
    }
}

struct EStructDef<X, Y>(X, Y);

impl<X, Y> ETrait<u32, X> for EStructDef<X, Y> {}

struct EStructDef2<X, Y>(X, Y);

impl<X, Y> ETrait<u32, X> for EStructDef2<X, Y> {}

struct EStruct<X, Y>(X, Y);

impl<X, Y> ETrait<u32, X> for EStruct<X, Y> {
    fn e_fn<V, W>(&self, _: u32, _: X, _: V, _: W) {
        println!("inner {:?}", Self::e_fn::<V, W>.get_paint_id());
    }
}
struct EStruct2<X, Y>(X, Y);

impl<X, Y> ETrait<u32, X> for EStruct2<X, Y> {
    fn e_fn<V, W>(&self, _: u32, _: X, _: V, _: W) {
        println!("inner {:?}", Self::e_fn::<V, W>.get_paint_id());
    }
}

fn dude_what() -> ! {
    println!("inner {:?}", dude_what.get_paint_id());
    panic!();
}

pub fn test_e() {
    let ss = "SS".to_string();
    let s = "S".to_string();

    println!("\nEStructDef");
    let e_struct_def_1 = EStructDef(&*s, 1.2f64);
    e_struct_def_1.e_fn(1, &ss, 2i64, 3u64);
    println!("outer {:?}", EStructDef::<&str, f64>::e_fn::<i64, u64>.get_paint_id());

    println!("\nEStructDef2");
    let e_struct_def_2 = EStructDef2(&*s, 1.2f64);
    e_struct_def_2.e_fn(1, &ss, 2i64, 3u64);
    println!("outer {:?}", EStructDef2::<&str, f64>::e_fn::<i64, u64>.get_paint_id());

    println!("\nEStruct");
    let e_struct = EStruct(&*s, 1.2f64);
    e_struct.e_fn(1, &ss, 2i64, 3u64);
    println!("outer {:?}", EStruct::<&str, f64>::e_fn::<i64, u64>.get_paint_id());

    println!("\nEStruct2");
    let e_struct2 = EStruct2(&*s, 1.2f64);
    e_struct2.e_fn(1, &ss, 2i64, 3u64);
    println!("outer {:?}", EStruct2::<&str, f64>::e_fn::<i64, u64>.get_paint_id());

    println!("\nEStruct2a");
    let e_struct2a = EStruct2(&*s, 1.2f32);
    e_struct2a.e_fn(1, &ss, 2i64, 3u64);
        println!("outer {:?}", EStruct2::<&str, f32>::e_fn::<i64, u64>.get_paint_id());

    println!("\nDude what");
    println!("outer {:?}", dude_what.get_paint_id());
//    dude_what();
}

