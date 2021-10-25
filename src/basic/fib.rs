fn calc(a: &mut i32, b: &mut i32, i: Option<&mut u8>) {
    let c = *a + *b;
    *a = *b;
    *b = c;
    match i {
        Some(i) => *i = *i + 1,
        None => (),
    };
}

#[allow(dead_code)]
pub fn fib_loop(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    loop {
        calc(&mut a, &mut b, Some(&mut i));
        println!("next: {:?}", b);
        if i >= n {
            break;
        }
    }
}

#[allow(dead_code)]
pub fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        calc(&mut a, &mut b, Some(&mut i));
        println!("next: {:?}", b);
    }
}

#[allow(dead_code)]
pub fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        calc(&mut a, &mut b, None);
        println!("next: {:?}", b);
    }
}
