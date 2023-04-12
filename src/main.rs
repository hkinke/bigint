use bigint::BigNumber;

fn main(){
	let a=BigNumber { mantissa:vec![77,67,13,41],sig:0 };
	let b=BigNumber { mantissa:vec![4,6,9],sig: 0 };
	let c=a+&b;
	
	let d=BigNumber { mantissa:vec![81,73,22,41],sig:0};
	let e=d-&b;
	
	let f=BigNumber { mantissa:vec![77,67,13,41],sig:0 };
	let g=BigNumber { mantissa:vec![4,6,9],sig: 0 };
	let h=g*&f;
	let m=BigNumber { mantissa:vec![4*77,4*67+6*77,4*13+6*67+9*77,4*41+6*13+9*67,6*41+9*13,9*41],sig:0};
	
	println!("{c:?}");
	println!("{e:?}");
	println!("{h:?}");
	println!("{m:?}");
}