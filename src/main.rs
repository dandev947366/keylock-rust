#[derive(Clone, Copy, Debug)]
enum ProtectedLocation{
    All,
    Office,
    Warehouse
}

impl ProtectedLocation{
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database{
    fn connect() -> Result<Self, String>{
        Ok(Database)
    }
    fn find_employee(&self, name:&str) -> Result<Employee, String>{
        match name{
            "Dan" => Ok(Employee {name: "Dan".to_string()}),
            "Brandy" => Ok(Employee {name: "Brandy".to_string()}),
            "Lucy" => Ok(Employee {name: "Lucy".to_string()}),
            _ => Err(String::from("employee not found"))
        }
    }
    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String>{
        match employee.name.as_str(){
            "Dan" => Ok(KeyCard {access_level:1000}),
            "Brandy" => Ok(KeyCard {access_level:500}),
            _ => Err(format!("{} doesn't have a keycard", employee.name))            
        }
    }
}
#[derive(Clone, Debug)]
struct Employee{
    name: String
}

#[derive(Debug)]
struct KeyCard{
    access_level: u16
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus{
    Allow,
    Deny
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation
) -> Result<AuthorizationStatus, String>{
    let db = Database::connect()?;
    let employee = db.find_employee(employee_name)?;
    let keycard = db.get_keycard(&employee)?;
    if keycard.access_level >= location.required_access_level(){
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main(){
    let dan_authorized = authorize("Dan", ProtectedLocation::Warehouse);
    let brandy_authorized = authorize("Brandy", ProtectedLocation::All);
    let kate_authorized = authorize("Kate", ProtectedLocation::Office);
    
    println!("{dan_authorized:?}");
    println!("{brandy_authorized:?}");
    println!("{kate_authorized:?}");
}

