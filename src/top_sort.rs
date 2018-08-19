

use std::collections::BTreeMap;
use std::hash::Hash;
use std::collections::btree_map::Iter;

#[derive(Debug)]
pub struct MBTreeMap<K: Eq + PartialEq + Hash, String :Clone>{
    map: BTreeMap<K, Vec<String>>,
}
	 
impl<K, String> MBTreeMap<K, String> 
    where K : Eq + PartialEq + Hash + Ord, String: Clone{
    pub fn new() -> Self {
        MBTreeMap {
            map: BTreeMap::new(),
        }
    }
 
    pub fn iter(&self) -> Iter<K, Vec<String>> {
        self.map.iter()
         
    }
 
    pub fn insert(&mut self, k: K, v: String) {
        if self.map.contains_key(&k) {
                self.map.get_mut(&k).unwrap().push(v);
        } else {
            self.map.insert(k, vec![v]);
        }
    }
 
    pub fn get(&self, k: &K) -> Option<&Vec<String>> {
        self.map.get(&k)
    }

    pub fn len(&self) -> usize {
    	self.map.len()
    }

    pub fn split_off(&mut self,k: &K) -> MBTreeMap<K, String> {
    	MBTreeMap {
    		map: self.map.split_off(k)
    	}

    }

	pub fn flatten(&self) -> Vec<String> {

    	self.map.iter().flat_map(|(_k,v)|v.iter()).cloned().collect()
    }
 
    pub fn get_mut(&mut self, k: &K) -> Option<&mut Vec<String>> {
        self.map.get_mut(&k)
    }
}	

