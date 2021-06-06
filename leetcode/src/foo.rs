

pub struct Employee {
    name: String,
    id: i32,
}

pub struct EmployeeRecord {
    employees: Vec![],
}


impl Iterator  for EmployeeRecord {
    type Item = Employee;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.into {
            todo!()
        }
    }
}