#!/usr/bin/env -S run-cargo-script
// cargo-deps: chrono
#![feature(non_ascii_idents)]
fn main() {
let π = std::f64::consts::PI; let sin = |x:f64| x.sin(); let cos = |x:f64| x.cos(); let atan = |sin:f64,cos:f64| sin.atan2(cos); let asin = |x:f64| x.asin(); let acos = |x:f64| x.acos(); let tan = |x:f64| x.tan(); let rad = |x:f64| x.to_radians();
// Orbital parameters
let e = 0.01671123; // Earth eccentricity
let Ω = rad(348.74_f64); // Longitude of ascending node
let ω = rad(114.21); // Argument of perihelion
let ε = rad(23.439); //Obliquity of the ecliptic
let year = 365.2145; //Sidereal year [solar day]
let n = 2.*π/(year*24.*60.*60.); //Mean motion [rad/s]

// Time
let u = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64(); //Unix time
let t = u - 10957.5*24.*60.*60.; // J2000 time [s]
let m0 = rad(357.51716); //Mean anomaly at J2000
let gmst0 = 18.697; //J2000 hour angle of the vernal equinox [h]
let ratio = 1.+1./year; //Sidereal day per solar day
let gmst = gmst0 + ratio*t/60./60.; // Greenwich mean sidereal time [h]

// Positions
let m = m0 + n*t; //Mean anomaly
let vu = m + 2.*e*sin(m) + 5./4.*e*e*sin(2.*m); //True anomaly
let l = vu + Ω + ω; //Ecliptic longitude (Omega + omega = Longitude of the periapsis)
let λ = π + l; //Ecliptic longitude (geocentric)
let α = atan(cos(ε)*sin(λ), cos(ε)*cos(λ)); //Right ascension
let δ = asin(sin(ε)*sin(λ)); //Declination

// Hours
let (longitude, latitude) = (rad(-2.1875), rad(48.6993));
let h = gmst - (longitude + α)/π*12.; // Solar hour angle [h]
let w0 = acos(-tan(latitude)*tan(δ))/π*12.*60.*60.; // Half day length [s]
let noon = u-h%24.*60.*60.; // Solar noon in unix time [s]
for &s in &[noon-w0, noon, noon+w0] { println!("{}", chrono::NaiveDateTime::from_timestamp(s as i64,0)); } // 8:18 - 16:42
}
