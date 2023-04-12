use std::ops::{Add,Sub,Mul};

#[derive(Debug,PartialEq)]
pub struct BigNumber {
	pub mantissa:Vec<u32>,
	pub sig:i32
}

impl BigNumber {

	fn real_sub(&mut self,b:&BigNumber) {
		let mut report:u32=1;
		if self.mantissa.len() < b.mantissa.len() {
			self.mantissa.resize(b.mantissa.len(),0);
			for i in 0..self.mantissa.len() {
				let mut x:u64=self.mantissa[i] as u64;
				x+=(!b.mantissa[i]) as u64;
				x+=report as u64;
				report = (x >> 32) as u32;
				self.mantissa[i]=x as u32;
			}
			
		} else {
			for i in 0..b.mantissa.len(){
				let mut x:u64=self.mantissa[i] as u64;
				x+=(!b.mantissa[i]) as u64;
				x+=report as u64;
				report = (x >> 32) as u32;
				self.mantissa[i]=x as u32;
			}
			for i in b.mantissa.len()..self.mantissa.len() {
				let mut x:u64=self.mantissa[i] as u64;
				x+=(!0u32) as u64;
				x+=report as u64;
				report = (x >> 32) as u32;
				self.mantissa[i]=x as u32;
				
				
			}
			
		}
		// compute bit sign
		report= (((!0u32) as u64) + (report as u64)) as u32;
		report>>=31;
		if report > 0  {
			// change from two complement to sign + absolute value
			report=1;
			for i in 0..self.mantissa.len() {
				let mut x:u64=(!self.mantissa[i]) as u64;
				x+=report as u64;
				report = (x >> 32) as u32;
				self.mantissa[i]=x as u32;
			}
			self.sig=1;
		}
		
	}
	fn real_add(&mut self,b:&BigNumber) {
		let mut report:u32=0;
		if self.mantissa.len() < b.mantissa.len() {
			self.mantissa.resize(b.mantissa.len(),0);
			for i in 0..self.mantissa.len() {
				let mut x:u64=self.mantissa[i] as u64;
				x+=b.mantissa[i] as u64;
				x+=report as u64;
				report = (x >> 32) as u32;
				self.mantissa[i]=x as u32;
			}
			
		} else {
			for i in 0..b.mantissa.len(){
				let mut x:u64=self.mantissa[i] as u64;
				x+=b.mantissa[i] as u64;
				x+=report as u64;
				report = (x >> 32) as u32;
				self.mantissa[i]=x as u32;
			}
			for i in b.mantissa.len()..self.mantissa.len() {
				let mut x:u64=self.mantissa[i] as u64;
				x+=report as u64;
				report = (x >> 32) as u32;
				self.mantissa[i]=x as u32;
				if report == 0 {
					
					break;
				}
				
			}
			
		}
		
		if report > 0  {
			self.mantissa.push(report);
		}
	}
	fn mul_scalar(& self,b:u32) -> BigNumber {
		let mut report:u32=0;
		let mut result=BigNumber { mantissa: self.mantissa.clone(),sig:self.sig};
		for i in 0..self.mantissa.len() {
			let mut x:u64=self.mantissa[i] as u64;
			x*=b as u64;
			x+=report as u64;
			report = (x >> 32) as u32;
			result.mantissa[i]=x as u32;
		}
		
		if report > 0  {
			result.mantissa.push(report);
		}
		result
	}
	fn prepend_zeros(&mut self,i:usize) {
		
		if i==0 {
			return;
		}
		
		self.mantissa.resize(self.mantissa.len()+i,0);
		
		for j in 0..(self.mantissa.len()-i){
			let k=self.mantissa.len()-1-j;
			let l=k-i;
			self.mantissa[k]=self.mantissa[l];
		}
		for j in 0..i {
			self.mantissa[j]=0;
		}
	}
	fn real_mul(&mut self,b:&BigNumber) {
		let mut d=BigNumber {mantissa:vec![0],sig:0};
		let e=BigNumber { mantissa:self.mantissa.clone(),sig:0};
		for i in 0..b.mantissa.len() {
			let mut c=e.mul_scalar(b.mantissa[i]);
			c.prepend_zeros(i);
			d=d+&c;
		}
		self.mantissa=d.mantissa;
		
	}
	
}

impl Add<&BigNumber> for BigNumber {
	
	type Output=BigNumber;
	
	fn add(mut self:BigNumber,b:&BigNumber) -> BigNumber {
		
		if (self.sig == 0) && (b.sig == 0) {
			self.real_add(b);
		} else if (self.sig == 0) && (b.sig != 0) {
			self.real_sub(b);
		} else if (self.sig != 0) && (b.sig == 0) {
			self.real_sub(b);
			self.sig=if self.sig == 0 { 1} else {0};
		} else {
			self.real_add(b);
		}
		self
	}
}

impl Sub<&BigNumber> for BigNumber {
	type Output=BigNumber;
	fn sub(mut self:BigNumber,b:&BigNumber) -> BigNumber {
		
		if (self.sig == 0) && (b.sig == 0) {
			self.real_sub(b);
		} else if (self.sig == 0) && (b.sig != 0) {
			self.real_add(b);
		} else if (self.sig != 0) && (b.sig == 0) {
			self.real_add(b);
		} else {
			self.real_sub(b);
			self.sig=if self.sig == 0 { 1} else {0};
		}
		self
	}
}

impl Mul<&BigNumber> for BigNumber {
	type Output=BigNumber;
	fn mul(mut self:BigNumber,b:&BigNumber) -> BigNumber {
		
		self.real_mul(b);
		if (self.sig == 0) && (b.sig == 0) {
			self.sig=0;
		} else if (self.sig == 0) && (b.sig != 0) {
			self.sig=1;
		} else if (self.sig != 0) && (b.sig == 0) {
			self.sig=1;
		} else {
			self.sig=0;
		}
		self
	}
}

#[test]
fn test_add_simple(){
	let a=BigNumber { mantissa:vec![77,67,13,41],sig:0 };
	let b=BigNumber { mantissa:vec![4,6,9],sig: 0 };
	let c=a+&b;
	let d=BigNumber { mantissa:vec![81,73,22,41],sig:0};
	
	assert_eq!(c,d)
	
}

#[test]
fn test_add_imbalance(){
	let a=BigNumber { mantissa:vec![77,67],sig:0 };
	let b=BigNumber { mantissa:vec![4,6,9],sig: 0 };
	let c=a+&b;
	let d=BigNumber { mantissa:vec![81,73,9],sig:0};
	
	assert_eq!(c,d)
	
}

#[test]
fn test_add_balance(){
	let a=BigNumber { mantissa:vec![((1u64 << 32)-1) as u32,67,11],sig:0 };
	let b=BigNumber { mantissa:vec![4,6,9],sig: 0 };
	let c=a+&b;
	let d=BigNumber { mantissa:vec![3,74,20],sig:0};
	
	assert_eq!(c,d)
	
}

#[test]
fn test_add_imbalance_overflow(){
	let a=BigNumber { mantissa:vec![77,67],sig:0 };
	let b=BigNumber { mantissa:vec![4,((1u64 << 32)-1) as u32,6],sig: 0 };
	let c=a+&b;
	let d=BigNumber { mantissa:vec![81,66,7],sig:0};
	
	assert_eq!(c,d)
	
}

#[test]
fn test_add_balance_overflow(){
	let a=BigNumber { mantissa:vec![77,67,11],sig:0 };
	let b=BigNumber { mantissa:vec![4,6,((1u64 << 32)-1) as u32],sig: 0 };
	let c=a+&b;
	let d=BigNumber { mantissa:vec![81,73,10,1],sig:0};
	
	assert_eq!(c,d)
	
}

#[test]
fn test_add_simple_overflow(){
	let a=BigNumber { mantissa:vec![77,67,13,((1u64 << 32)-1) as u32],sig:0 };
	let b=BigNumber { mantissa:vec![4,6,((1u64 << 32)-1) as u32],sig: 0 };
	let c=a+&b;
	let d=BigNumber { mantissa:vec![81,73,12,0,1],sig:0};
	
	assert_eq!(c,d)
	
}

#[test]
fn test_add_imple_funny(){
	let a=BigNumber { mantissa:vec![1],sig:0 };
	let b=BigNumber { mantissa:vec![((1u64 << 32)-1) as u32,((1u64 << 32)-1) as u32,((1u64 << 32)-1) as u32],sig: 0 };
	let c=a+&b;
	let d=BigNumber { mantissa:vec![0,0,0,1],sig:0};
	
	assert_eq!(c,d)
	
}

#[test]
fn test_sub_simple(){
	let a=BigNumber { mantissa:vec![77,67,13,41],sig:0 };
	let b=BigNumber { mantissa:vec![4,6,9],sig: 0 };
	let d=BigNumber { mantissa:vec![81,73,22,41],sig:0};
	
	let c=d-&b;
	
	assert_eq!(a,c)
	
}

#[test]
fn test_sub_simple_neg(){
	let a=BigNumber { mantissa:vec![77,67,13,41],sig:1 };
	let b=BigNumber { mantissa:vec![4,6,9],sig: 0 };
	let d=BigNumber { mantissa:vec![81,73,22,41],sig:0};
	
	let c=b-&d;
	
	assert_eq!(a,c)
	
}

#[test]
fn test_prepend_zeros() {
	let mut a=BigNumber { mantissa:vec![77,67,13,41],sig:0 };
	let b=BigNumber { mantissa:vec![0,0,0,77,67,13,41],sig: 0 };
	let mut c=BigNumber { mantissa:vec![77,67,13,41],sig:0 };
	let d=BigNumber { mantissa:vec![77,67,13,41],sig: 0 };
	
	a.prepend_zeros(3);
	c.prepend_zeros(0);
	
	
	assert_eq!(a,b);
	assert_eq!(c,d);
	
}

#[test]
fn test_mul_scalar() {
	let a=BigNumber { mantissa:vec![77,67,13,1u32<<31],sig:0 };
	let b=BigNumber { mantissa:vec![77*55,67*55,13*55,((1u64<<31)*55) as u32,(((1u64<<31)*55)>>32) as u32],sig: 0 };
	
	let c=a.mul_scalar(55);
	
	assert_eq!(b,c);
	
}

#[test]
fn test_mul() {
	let f=BigNumber { mantissa:vec![77,67,13,41],sig:1 };
	let g=BigNumber { mantissa:vec![4,6,9],sig: 1 };
	let h=g*&f;
	let m=BigNumber { mantissa:vec![4*77,4*67+6*77,4*13+6*67+9*77,4*41+6*13+9*67,6*41+9*13,9*41],sig:0};
	
	assert_eq!(m,h);
}