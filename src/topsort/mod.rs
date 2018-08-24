mod decimal;
mod multi_btreemap;

use topsort::multi_btreemap::MBTreeMap;
use topsort::decimal::Decimal;

pub enum OrderType {
	DEFAULT,
	REVERSE
}

pub struct TopSort {
    ordering: OrderType,
    desired_resuts: usize,
    tree: MBTreeMap<Decimal,String>,
    trim_ratio:usize
}

impl TopSort {
	pub fn new(ordering: OrderType ,desired_resuts:usize) -> TopSort {
		TopSort {
			ordering,
			desired_resuts,
			tree: MBTreeMap::new(),
			trim_ratio: 2
		}
	}

	pub fn add(&mut self,key:&str,value:&str) -> () {
		let decimal_key = key.parse::<Decimal>().unwrap();
		self.tree.insert(decimal_key,value.to_string());
		if self.tree.len() > self.desired_resuts * self.trim_ratio {	
			self.trim_tree();
		}
	}

	fn trim_tree(&mut self) -> (){
		//println!("{:?}", "Trimming tree");
		match self.ordering {
		    OrderType::DEFAULT => self.trim_tree_top(),
		    OrderType::REVERSE => self.trim_tree_bottom()
		}
	}

	fn trim_tree_top(&mut self) -> (){
		if self.desired_resuts < self.tree.len(){
			return;
		}
		let ammount_to_prune = self.tree.len() - self.desired_resuts;
		let (&splitter,_) = self.tree.iter().nth(ammount_to_prune).unwrap();
		let tree_top = self.tree.split_off(&splitter);
		self.tree = tree_top;
	}

	fn trim_tree_bottom(&mut self) -> (){
		if self.desired_resuts < self.tree.len(){
			return;
		}
		let ammount_to_prune = self.tree.len() - self.desired_resuts;
		let (&splitter,_) = self.tree.iter().rev().nth(ammount_to_prune-1).unwrap();
		let _tree_top = self.tree.split_off(&splitter);
		assert_eq!(self.tree.len(),self.desired_resuts);
	}

	pub fn get_result(&self) -> Vec<String> {
		let results= self.tree.flatten();
		let should_skip = if results.len() > self.desired_resuts {
			results.len() - self.desired_resuts
		}else{
			0
		};
		
		match self.ordering {
		    OrderType::DEFAULT => results.iter().skip(should_skip).cloned().collect(),
		    OrderType::REVERSE => results.iter().rev().skip(should_skip).cloned().collect()
		}

	}
}