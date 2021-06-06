

#[derive(Clone)]
pub struct Employee {
    name: String,
    id: i32,
}

pub struct EmployeeRecord {
    employees: Vec<Employee>,
    idx: usize,
}


impl Iterator  for EmployeeRecord {
    type Item = Employee;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > self.idx {
            self.idx += 1; 
            let o: Employee = self.employees[self.idx].clone();
            Some(o);
        } else {
            None
        }
    }
}