mod decimal;
mod multi_btreemap;

pub mod top_sort {
	use std::num::ParseIntError;
	use topsort::decimal::Decimal;
	use topsort::multi_btreemap::MBTreeMap;
	use csv::ByteRecord;

	#[derive(Clone)]
	pub enum OrderType {
		DEFAULT,
		REVERSE,
	}

	pub struct TopSortEntry<'a> {
		key: Decimal,
		byte_record: &'a ByteRecord,
	}

	impl<'a> TopSortEntry<'a> {
		pub fn new(key: &str, line: &'a ByteRecord) -> Result<TopSortEntry<'a>, ParseIntError> {
			Ok(TopSortEntry {
				key: key.parse::<Decimal>()?,
				byte_record: line,
			})
		}
	}

	pub struct TopSort {
		ordering: OrderType,
		desired_resuts: usize,
		tree: MBTreeMap<Decimal, ByteRecord>,
		trim_ratio: usize,
		bound: Option<Decimal>,
	}

	impl TopSort {

		pub fn new(ordering: OrderType, desired_resuts: usize) -> TopSort {
			TopSort {
				ordering,
				desired_resuts,
				tree: MBTreeMap::new(),
				trim_ratio: 2,
				bound: None,
			}
		}

		fn in_bound(&self, value: Decimal) -> bool {
			// match self.ordering {
			// 	OrderType::DEFAULT => self.tree.iter().nth(0).map_or(true,|(key,_)| key < &value),
			// 	OrderType::REVERSE => self.tree.iter().last().map_or(true,|(key,_)| key > &value),
			// }
			match self.ordering {
				OrderType::DEFAULT => self.bound.map_or(true, |bound| bound <= value),
				OrderType::REVERSE => self.bound.map_or(true, |bound| bound >= value),
			}
		}

		fn update_bound(&mut self) {
			self.bound = match self.ordering {
				OrderType::DEFAULT => self.tree.iter().nth(0).map(|x| x.0).cloned(),
				OrderType::REVERSE => self.tree.iter().last().map(|x| x.0).cloned(),
			}
		}

		pub fn add(&mut self, entry: TopSortEntry) -> () {
			if !self.in_bound(entry.key) {
				return;
			}
			let decimal_key = entry.key;
			self.tree.insert(decimal_key, entry.byte_record.clone());
			if self.bound.is_none() {
				self.update_bound();
			}
			if self.tree.len() > self.desired_resuts * self.trim_ratio {
				self.trim_tree();
			}
		}

		fn trim_tree(&mut self) -> () {
			match self.ordering {
				OrderType::DEFAULT => self.trim_tree_top(),
				OrderType::REVERSE => self.trim_tree_bottom(),
			}
			self.update_bound();
		}

		fn trim_tree_top(&mut self) -> () {
			if self.desired_resuts > self.tree.len() {
				return;
			}
			let ammount_to_prune = self.tree.len() - self.desired_resuts;
			let (&splitter, _) = self.tree.iter().nth(ammount_to_prune).unwrap();
			let tree_top = self.tree.split_off(&splitter);
			self.tree = tree_top;
			assert_eq!(self.tree.len(), self.desired_resuts);
		}

		fn trim_tree_bottom(&mut self) -> () {
			if self.desired_resuts > self.tree.len() {
				println!("{}", "early exit");
				return;
			}
			let ammount_to_prune = self.tree.len() - self.desired_resuts;
			let (&splitter, _) = self.tree.iter().rev().nth(ammount_to_prune - 1).unwrap();
			let _tree_top = self.tree.split_off(&splitter);
		}

		pub fn get_result(&self) -> Vec<ByteRecord> {
			let results = self.tree.flatten();
			let should_skip = if results.len() > self.desired_resuts {
				results.len() - self.desired_resuts
			} else {
				0
			};

			match self.ordering {
				OrderType::DEFAULT => results.iter().skip(should_skip).cloned().collect(),
				OrderType::REVERSE => results.iter().rev().skip(should_skip).cloned().collect(),
			}
		}
	}
}
