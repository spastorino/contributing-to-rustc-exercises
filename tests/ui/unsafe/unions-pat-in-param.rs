union U {
    a: &'static i32,
    b: usize,
}

fn fun(U { a }: U) {
    dbg!(*a);
}

fn main() {
    fun(U { b: 0 });

    let closure = |U { a }| {
        dbg!(*a);
    };
    closure(U { b: 0 });
}

