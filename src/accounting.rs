use std::collections::HashMap;

pub trait Summary {
	fn summary(&self) -> String;
	fn to_str(&serl) -> &str;
}

#[derive(Debug)]
pub struct Department {
	name: String,
	personal: Vec<String>
}

impl Department {
	fn new(name: String) -> Department {
		Department {
			name: name,
			personal: Vec::new()
		}
	}

	pub fn add_employee(&mut self, name: &str) {
		self.personal.push(String::from(name));
	}

	pub fn remove_employee(&mut self, name: &str) {
		self.personal.retain(|e| *e != String::from(name));
	}
}

pub struct Office {
	name: String,
	departmens: HashMap<String, Department>
}

impl Summary for Office {
	fn summary(&self) -> String {
		format!("{} {:?}", self.name, self.departmens)
	}
	fn to_str(&self) -> &'static str {
		
	}
}

impl Office {
	pub fn new(name: &str) -> Office {
		Office {
			name: String::from(name),
			departmens: HashMap::new()
		}
	}

	pub fn get_department(&mut self, name: &str) -> Option<&mut Department> {
		self.departmens.get_mut(name)
	}

	pub fn add_department(&mut self, name: &str) {
		self.departmens.insert(String::from(name), Department::new(String::from(name)));
	}

	pub fn remove_department(&mut self, name: &str) {
		self.departmens.remove(name);
	}
}

pub trait Storage {

}