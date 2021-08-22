#!/usr/bin/env run-cargo-script
#![feature(destructuring_assignment,iter_zip,format_args_capture)]
use std::{iter::{from_fn,zip}, env::args, fs::read, str::from_utf8};
fn split(mut s: &[u8], separator: u8) -> impl Iterator<Item=&[u8]> { from_fn(move || {
	let e; (e, s) = s.into_iter().enumerate().	find({let mut depth = 0; move |(_,&c)| {
		if c == b'[' { depth+=1; }
		if c == b']' { depth-=1; }
		c == separator && depth == 0
	}}).map(|(i,_)| (&s[..i], &s[i+1..])).unwrap_or((s, &[]));
	(!e.is_empty()).then(|| e)
})}
fn array(s:&[u8]) -> Box<[&[u8]]> { split(from_utf8(s).unwrap().trim().strip_prefix('[').unwrap().strip_suffix(']').unwrap().as_bytes(),b',').collect() }

fn main() { match 1 {
	1 => {
		let parse = |path| -> Box<[f64]> {
			let ref s = read(path).expect(path);
			array(s).iter().map(|s| from_utf8(s).unwrap().trim().parse().unwrap()).collect()
		};
		let [a, b] = [parse(&args().skip(1).next().unwrap()), parse(&args().skip(2).next().unwrap())];
		let (i, (a,b), e) =
		zip(&*a,&*b).enumerate()
		.map(|(i,(&a,&b))| (i, (a,b), f64::abs(a-b)/f64::min(a,b)))
		.reduce(|a,b| if a.2 > b.2 { a } else { b }).unwrap();
		println!("{i} {a} {b} {e:e}");
	},
	2 => {
		let parse = |path| -> Box<[Box<[f64]>]> {
			let ref s = read(path).expect(path);
			array(s).iter().map(|s| array(s).iter().map(|s| from_utf8(s).unwrap().trim().parse().unwrap()).collect()).collect()
		};
		let [a, b] = [parse(&args().skip(1).next().unwrap()), parse(&args().skip(2).next().unwrap())];
		let ((i,j), (a,b), e) =
		zip(&*a,&*b).enumerate().map(|(i,(a,b))|
		 zip(&**a,&**b).enumerate()
		 .map(move |(j,(&a,&b))| ((i,j), (a,b), f64::abs(a-b)/f64::min(a,b)))
		).flatten()
		.reduce(|a,b| if a.2 > b.2 { a } else { b }).unwrap();
		println!("{i}:{j} {a} {b} {e:e}");
	},
	3 => {
	let parse = |path| -> Box<[Box<[Box<[f64]>]>]> {
			let ref s = read(path).expect(path);
			array(s).iter().map(|s| array(s).iter().map(|s| array(s).iter().map(|s| from_utf8(s).unwrap().trim().parse().unwrap()).collect()).collect()).collect()
		};
		let [a, b] = [parse(&args().skip(1).next().unwrap()), parse(&args().skip(2).next().unwrap())];
		let ((i,j,k), (a,b), e) =
		zip(&*a,&*b).enumerate().map(|(i,(a,b))|
			zip(&**a,&**b).enumerate().map(move |(j,(a,b))|
			 zip(&**a,&**b).enumerate()
			 .map(move |(k,(&a,&b))| ((i,j,k), (a,b), f64::abs(a-b)/f64::min(a,b)))
			).flatten()
		).flatten()
		.reduce(|a,b| if a.2 > b.2 { a } else { b }).unwrap();
		println!("{i}:{j}:{k} {a} {b} {e:e}");
	},
	_ => unreachable!()
}}
