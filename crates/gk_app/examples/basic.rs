use anymap::AnyMap;
use hashbrown::HashMap;
use std::any::Any;
use std::any::TypeId;

trait Plugin {}

pub struct Storage {
    a: i32,
    b: u32,
    c: f32,
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Storage {
    fn new() -> Self {
        Self { a: 0, b: 0, c: 0.0, map: HashMap::default() }
    }

    fn add<T: Plugin + 'static>(&mut self, plugin: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(plugin));
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let t = self.map.get_mut(&TypeId::of::<T>())?;
        t.downcast_mut()
    }
}

trait FromStorage<'a, P> {
    fn from_s(s: &'a mut Storage) -> P;
}

impl<'a> FromStorage<'a, &'a mut i32> for &'a mut i32 {
    fn from_s(s: &'a mut Storage) -> &'a mut i32 {
        let ptr = &mut s.a as *mut _;
        unsafe {
            &mut *ptr
        }
    }
}

impl<'a> FromStorage<'a, &'a i32> for &'a i32 {
    fn from_s(s: &'a mut Storage) -> &'a i32 {
        &s.a
    }
}

impl<'a> FromStorage<'a, i32> for i32 {
    fn from_s(s: &'a mut Storage) -> i32 {
        s.a
    }
}

impl<'a, P: Plugin + 'static> FromStorage<'a, &'a mut P> for &'a mut P {
    fn from_s(s: &'a mut Storage) -> &'a mut P {
        let key = TypeId::of::<P>();
        /*let v: &mut P = s.map.get_mut(&key).unwrap();
        let ptr = v as *mut _;
        unsafe {
            &mut *ptr
        }*/
        s.get_mut::<P>().unwrap()
    }
}

impl<'a, P: Plugin + 'static> FromStorage<'a, &'a P> for &'a P {
    fn from_s(s: &'a mut Storage) -> &'a P {
        let key = TypeId::of::<P>();
        let t = *s.map.get(&key).as_ref().unwrap();
        t.downcast_ref().unwrap()
    }
}

fn main() {
    println!("-.-");
    let mut s = Storage {
        a: 1234,
        b: 4321,
        c: 2134.0,
        ..Storage::new()
    };
    dispatch(&mut s, |a: i32| println!("1: a -> {a}"));
    dispatch(&mut s, |a: &mut i32| {
        println!("2: a -> {}", a);
        *a = 9999;
    });
    dispatch(&mut s, |a: &i32| println!("3: a -> {}", a));
}

fn dispatch<'a, H, T>(s: &'a mut Storage, mut handler: H)
where
    H: Handler<'a, T>,
{
    handler.call(s);
}

pub trait Handler<'a, T> {
    fn call(&mut self, s: &'a mut Storage);
}

impl<'a, Fun> Handler<'a, ()> for Fun
where
    Fun: FnMut(),
{
    fn call(&mut self, s: &'a mut Storage) {
        (self)();
    }
}

impl<'a, Fun, A> Handler<'a, (A,)> for Fun
where
    Fun: FnMut(A),
    A: FromStorage<'a, A>,
{
    fn call(&mut self, s: &'a mut Storage) {
        let (a_v,) = (A::from_s(s),);
        (self)(a_v);
    }
}
