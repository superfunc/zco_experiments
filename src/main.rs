use rand::Rng;
use std::time::*;
use std::vec::Vec;

#[macro_use]
extern crate lazy_static;

// Some simple loop based benchmarks aimed at isolating the cost of wrap/unwrap around
// certain optimized types inside of Rust.

// op: generics
//
// --------------------------------------------

#[derive(Debug, Default, Copy, Clone)]
struct Gen<T, V> {
    t: T,
    v: V
}

impl Gen<i64, f64> {
    fn f(&self) -> i64 {
        self.t + (self.v as i64)
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Ungen {
    t: i64,
    v: f64 
}

impl Ungen {
    fn f(&self) -> i64 {
        self.t + (self.v as i64)
    }
}

const gen_col_size : usize = 10000;

lazy_static! {
    static ref Gens: Vec<Gen<i64, f64>> = {
        let mut v = vec![Gen { t: 0, v: 0.0 }];
        for i in 0..gen_col_size {
            v.push( Gen { t: i as i64, v: i as f64} );
        }

        v
    };

    static ref UnpackedGens: Vec<Ungen> = {
        let mut v = vec![Ungen { t: 0, v: 0.0 }];
        for i in 0..gen_col_size {
            v.push( Ungen { t: i as i64, v: i as f64} );
        }

        v
    };

}

fn op_generics_using_zco() -> i64 { 
    let mut sum = 0;
    for i in 0..gen_col_size {
        sum += (Gens[i].t + Gens[i].v as i64);
    }

    sum

}

fn op_generics_without_zco() -> i64 { 
    let mut sum = 0;
    for i in 0..gen_col_size {
        sum += (UnpackedGens[i].t + UnpackedGens[i].v as i64);
    }

    sum
}

// op: newtypes
//
// --------------------------------------------

const newtype_col_size: usize = 10000;

#[derive(Debug, Default, Copy, Clone)]
struct Foo(i64);

#[derive(Debug, Default, Copy, Clone)]
struct Bar((i64, i64, f32));

lazy_static! {
    static ref WrappedFoos: Vec<Foo> = {
        let mut v = vec![Foo(0)];
        for i in 0..newtype_col_size {
            v.push(Foo(i as i64));
        }

        v
    };
    static ref WrappedBars: Vec<Bar> = {
        let mut v = vec![Bar((0, 0, 0.0))];
        for i in 0..newtype_col_size {
            v.push(Bar((i as i64, i as i64, i as f32)));
        }

        v
    };
    static ref UnwrappedFoos: Vec<i64> = {
        let mut v = vec![0];
        for i in 0..newtype_col_size {
            v.push(i as i64);
        }

        v
    };
    static ref UnwrappedBars: Vec<(i64, i64, f32)> = {
        let mut v = vec![(0, 0, 0.0)];
        for i in 0..newtype_col_size {
            v.push((i as i64, i as i64, i as f32));
        }

        v
    };
}

fn op_newtypes_using_zco() -> i64 {
    let mut sum = 0;
    for i in 0..newtype_col_size {
        sum += WrappedFoos[i].0;
        sum += (WrappedBars[i].0).0 + (WrappedBars[i].0).1 + ((WrappedBars[i].0).2 as i64);
    }

    sum
}

fn op_newtypes_without_zco() -> i64 {
    let mut sum = 0;
    for i in 0..newtype_col_size {
        sum += UnwrappedFoos[i];
        sum += (UnwrappedBars[i]).0 + (UnwrappedBars[i]).1 + ((UnwrappedBars[i]).2 as i64);
    }

    sum
}

// op: option
//
// --------------------------------------------
const option_col_size: usize = 10000;

lazy_static! {
    static ref Options: Vec<Option<i64>> = {
        let mut v = vec![Some(0)];
        for i in 0..option_col_size {
            v.push(Some(i as i64));
        }

        v
    };
    static ref Defaulted: Vec<i64> = {
        let mut v = vec![0 as i64];
        for i in 0..option_col_size {
            v.push(i as i64);
        }

        v
    };
}

fn op_option_using_zco() -> i64 {
    let mut sum = 0;
    for i in 0..option_col_size {
        sum += Options[i].unwrap_or(0);
    }

    sum
}

fn op_option_without_zco() -> i64 {
    let mut sum = 0;
    for i in 0..option_col_size {
        sum += Defaulted[i];
    }

    sum
}

// op: result
//
// --------------------------------------------
const result_col_size: usize = 10000;

lazy_static! {
    static ref Results: Vec<Result<i64, &'static str>> = {
        let mut v = vec![Ok(0)];
        for i in 0..result_col_size {
            v.push(Ok(i as i64));
        }

        v
    };
    static ref DefaultedRes: Vec<i64> = {
        let mut v = vec![0 as i64];
        for i in 0..result_col_size {
            v.push(i as i64);
        }

        v
    };
}

fn op_result_using_zco() -> i64 {
    let mut sum = 0;
    for i in 0..result_col_size {
        sum += Results[i].unwrap_or(0);
    }

    sum
}

fn op_result_without_zco() -> i64 {
    let mut sum = 0;
    for i in 0..result_col_size {
        sum += DefaultedRes[i];
    }

    sum
}

// op: basic iteration
//
// --------------------------------------------

const iteration_col_size: usize = 10000;

lazy_static! {
    static ref BasicIterCol: Vec<i64> = {
        let mut v = vec![0 as i64];
        for i in 0..iteration_col_size {
            v.push(i as i64);
        }

        v
    };
}

fn op_basic_iteration_using_zco() -> i64 {
    let sum: i64 = BasicIterCol.iter().sum();
    sum
}

fn op_basic_iteration_without_zco() -> i64 {
    let mut sum = 0;
    for i in 0..iteration_col_size {
        sum += BasicIterCol[i];
    }

    sum
}

// op: composed iteration
//
// --------------------------------------------
fn op_composed_iteration_using_zco() -> i64 { 
    let sum: i64 = BasicIterCol.iter().map(|i| i + 42).take(5000).sum();
    sum   
}

fn op_composed_iteration_without_zco() -> i64 { 
    let mut sum = 0;
    for i in 0..5000 {
        sum += BasicIterCol[i]+42;
    }

    sum
}

fn bench<F: Fn() -> i64>(iterations: usize, op: &F, name: &str) {
    let mut times: Vec<f64> = vec![];

    // warm up
    for i in 0..1000 {
        let _ = op();
    }

    for i in 0..iterations {
        let now = Instant::now();
        let res = op();
        let later = Instant::now();
        let elapsed = Duration::from(later - now);
        times.push(elapsed.as_micros() as f64);
    }

    println!(
        "Op {} took an average of {}μs",
        name,
        times.iter().sum::<f64>() / iterations as f64
    );
}

fn using_zco(iterations: usize) {
    let names = vec![
        "basic_iterations",
        "composing_iterators",
        "newtypes",
        "option",
        "result",
        "generics",
    ];

    let operations = [
        op_basic_iteration_using_zco,
        op_composed_iteration_using_zco,
        op_newtypes_using_zco,
        op_option_using_zco,
        op_result_using_zco,
        op_generics_using_zco,
    ];

    for i in 0..names.len() {
        bench(iterations, &operations[i], names[i]);
    }
}

fn without_zco(iterations: usize) {
    let names = vec![
        "basic_iterations",
        "composing_iterators",
        "newtypes",
        "option",
        "result",
        "generics",
    ];

    let operations = [
        op_basic_iteration_without_zco,
        op_composed_iteration_without_zco,
        op_newtypes_without_zco,
        op_option_without_zco,
        op_result_without_zco,
        op_generics_without_zco,
    ];

    for i in 0..names.len() {
        bench(iterations, &operations[i], names[i]);
    }
}

fn main() {
    let iterations = 10000;

    println!("Using ZCO:");
    using_zco(iterations);

    println!("\n\nHand written:");
    without_zco(iterations);
}
