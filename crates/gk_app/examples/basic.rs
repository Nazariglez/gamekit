pub struct S {
    a: i32,
    b: u32,
    c: f32,
}

trait FromS<'a, P> {
    fn from_s(s: &'a mut S) -> P;
}

impl<'a> FromS<'a, &'a mut i32> for &'a mut i32 {
    fn from_s(s: &'a mut S) -> &'a mut i32 {
        let ptr = &mut s.a as *mut _;
        unsafe {
            &mut *ptr
        }
    }
}

impl<'a> FromS<'a, &'a i32> for &'a i32 {
    fn from_s(s: &'a mut S) -> &'a i32 {
        &s.a
    }
}

impl<'a> FromS<'a, i32> for i32 {
    fn from_s(s: &'a mut S) -> i32 {
        s.a
    }
}

fn main() {
    println!("-.-");
    let mut s = S {
        a: 1234,
        b: 4321,
        c: 2134.0,
    };
    dispatch(&mut s, |a: i32| println!("1: a -> {a}"));
    dispatch(&mut s, |a: &mut i32| {
        println!("2: a -> {}", a);
        *a = 9999;
    });
    dispatch(&mut s, |a: &i32| println!("3: a -> {}", a));
}

fn dispatch<'a, H, T>(s: &'a mut S, mut handler: H)
where
    H: Handler<'a, T>,
{
    handler.call(s);
}

pub trait Handler<'a, T> {
    fn call(&mut self, s: &'a mut S);
}

impl<'a, Fun> Handler<'a, ()> for Fun
where
    Fun: FnMut(),
{
    fn call(&mut self, s: &'a mut S) {
        (self)();
    }
}

impl<'a, Fun, A> Handler<'a, (A,)> for Fun
where
    Fun: FnMut(A),
    A: FromS<'a, A>,
{
    fn call(&mut self, s: &'a mut S) {
        let (a_v,) = (A::from_s(s),);
        (self)(a_v);
    }
}
