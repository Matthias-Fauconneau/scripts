#!/usr/bin/env run-cargo-script
#![feature(iter_zip,format_args_capture)]
fn main() {
	use std::{iter::zip, env::args, fs::read, str::from_utf8};
	let parse = |path| -> Box<[Box<[f64]>]> {
		let ref s = read(path).expect(path);
		fn split(mut s: &[u8], separator: u8) -> impl Iterator<Item=&[u8]> {
			std::iter::from_fn(move || {
				let mut depth = 0;
				let mut next = 0;
				let end = loop {
					if next >= s.len() { break s.len(); }
					match s[next] {
						b'[' => { depth+=1; },
						b']' => { depth-=1; },
						c if c == separator && depth == 0 => { let end=next; next+=1; break end; }
						_ => {}
					}
					next += 1;
				};
				let e = &s[..end];
				s = &s[next..];
				(!e.is_empty()).then(|| e)
			})
		}
		fn array(s:&[u8]) -> Box<[&[u8]]> { split(from_utf8(s).unwrap().trim().strip_prefix('[').unwrap().strip_suffix(']').unwrap().as_bytes(),b',').collect() }
		array(s).iter().map(|s| array(s).iter().map(|s| from_utf8(s).unwrap().trim().parse().unwrap()).collect()).collect()
	};
	let [a, b] = [parse(&args().skip(1).next().unwrap()), parse(&args().skip(2).next().unwrap())];
	for (i,(a,b)) in zip(&*a,&*b).enumerate() { for (j,(&a,&b)) in zip(&**a,&**b).enumerate() { let e = f64::abs(a-b)/f64::min(a,b); assert!(e<=1e-3,"{i}:{j}: {a} {b} {e:e}"); } }
}
