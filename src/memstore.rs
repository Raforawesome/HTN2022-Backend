#![allow(clippy::redundant_closure, unused)]
use std::sync::{LazyLock, Mutex};
use std::collections::HashMap;

type BucketStorage = HashMap<String, Vec<(String, String)>>;
static BUCKETS: LazyLock<Mutex<BucketStorage>> = LazyLock::new(|| {
	Mutex::new(HashMap::new())
});


pub fn add_to_bucket(location: String, course: String, name: String) {
	let mut guard = BUCKETS.lock().unwrap();
	let loc_arr = guard
		.entry(location)
		.or_insert_with(|| Vec::new());
	loc_arr.push((course, name));
}

pub fn get_by_location(location: &str) -> Option<Vec<(String, String)>> {
	let mut guard = BUCKETS.lock().unwrap();
	#[allow(clippy::significant_drop_in_scrutinee)]
	match guard.entry(location.to_string()) {
    	std::collections::hash_map::Entry::Occupied(v) => Some(v.get().clone()),
    	std::collections::hash_map::Entry::Vacant(_) => None,
	}
}