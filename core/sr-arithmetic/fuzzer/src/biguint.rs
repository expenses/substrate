use honggfuzz::fuzz;
use sr_arithmetic::BigUint;
use num_traits::{ops::checked::CheckedSub, ToPrimitive};

fn main() {
	loop {
		fuzz!(|data: (Vec<u32>, Vec<u32>)| {
			let (digits_u, digits_v) = data;

			run_with_data_set(4, &digits_u, &digits_v, |u, v| {
				let ue = u.to_u128().unwrap();
				let ve = v.to_u128().unwrap();
				assert_eq!(u.cmp(&v), ue.cmp(&ve));
				assert_eq!(u.eq(&v), ue.eq(&ve));
			});

			run_with_data_set(3, &digits_u, &digits_v, |u, v| {
				let expected = u.clone().to_u128().unwrap() + v.clone().to_u128().unwrap();
				let t = u.clone() + &v;
				assert_eq!(
					t.clone().to_u128().unwrap(), expected,
					"{:?} + {:?} ===> {:?} != {:?}", u, v, t, expected,
				);
			});

			run_with_data_set(4, &digits_u, &digits_v, |u, v| {
				let expected = u.clone().to_u128()
					.unwrap()
					.checked_sub(v.clone().to_u128().unwrap());
				let t = u.clone().checked_sub(&v);
				if expected.is_none() {
					assert!(t.is_none())
				} else {
					let t = t.unwrap();
					let expected = expected.unwrap();
					assert_eq!(
						t.to_u128().unwrap(), expected,
						"{:?} - {:?} ===> {:?} != {:?}", u, v, t, expected,
					);
				}
			});

			run_with_data_set(2, &digits_u, &digits_v, |u, v| {
				let expected = u.clone().to_u128().unwrap() * v.clone().to_u128().unwrap();
				let t = u.clone() * &v;
				assert_eq!(
					t.to_u128().unwrap(), expected,
					"{:?} * {:?} ===> {:?} != {:?}", u, v, t, expected,
				);
			});

			run_with_data_set(4, &digits_u, &digits_v, |u, v| {
				let ue = u.to_u128().unwrap();
				let ve = v.to_u128().unwrap();
				if ve == 0 {
					return;
				}
				let (q, r) = (ue / ve, ue % ve);
				let qq = u.clone() / &v;
				let rr = u.clone() % &v;
				assert_eq!(
					qq.to_u128().unwrap(), q,
					"{:?} / {:?} ===> {:?} != {:?}", u, v, qq, q,
				);
				assert_eq!(
					rr.to_u128().unwrap(), r,
					"{:?} % {:?} ===> {:?} != {:?}", u, v, rr, r,
				);
			});
		});
	}
}

fn run_with_data_set<F>(
	max_limbs: usize,
	digits_u: &[u32], digits_v: &[u32],
	assertion: F,
)
	where
		F: Fn(BigUint, BigUint) -> (),
{
	// Ensure that `1 <= num_digits < max_limbs`.
	let valid = value_between(digits_u.len(), 1, max_limbs) &&
		value_between(digits_v.len(), 1, max_limbs);
	if !valid {
		return;
	}

	let u = BigUint::new(digits_u.to_vec());
	let v = BigUint::new(digits_v.to_vec());
	// this is easier than using lldb
	// println!("u: {:?}, v: {:?}", u, v);

	assertion(u, v)
}

fn value_between(value: usize, min: usize, max: usize) -> bool {
	min <= value && value <= max
}