use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, Stdin, Stdout, Write};
use std::process::exit;

#[derive(Debug)]
enum Command {
    AddDepartment {
        department: String,
    },
    AddEmployee {
        department: String,
        name: String,
    },
    Exit,
    ListAll,
    ListDepartment {
        department: String,
    },
    Menu,
    MoveEmployee {
        src_department: String,
        dst_department: String,
        name: String,
    },
    RemoveDepartment {
        department: String,
    },
    RemoveEmployee {
        department: String,
        name: String,
    },
    RenameDepartment {
        old_department: String,
        new_department: String,
    },
    RenameEmployee {
        department: String,
        old_name: String,
        new_name: String,
    },
}

impl Command {
    fn new_add_department(department: String) -> Result<Command, String> {
        match department.as_str() {
            "" => Err("missing DEPARTMENT argument".to_string()),
            _ => Ok(Self::AddDepartment { department }),
        }
    }

    fn new_add_employwee(department: String, name: String) -> Result<Command, String> {
        match (department.as_str(), name.as_str()) {
            ("", _) => Err("missing DEPARTMENT argument".to_string()),
            (_, "") => Err("missing NAME argument".to_string()),
            _ => Ok(Self::AddEmployee { department, name }),
        }
    }

    fn new_list_department(department: String) -> Result<Command, String> {
        match department.as_str() {
            "" => Err("missing DEPARTMENT argument".to_string()),
            _ => Ok(Self::ListDepartment { department }),
        }
    }

    fn new_move_employee(
        src_department: String,
        dst_department: String,
        name: String,
    ) -> Result<Command, String> {
        match (
            src_department.as_str(),
            dst_department.as_str(),
            name.as_str(),
        ) {
            ("", _, _) => Err("missing SRC_DEPARTMENT argument".to_string()),
            (_, "", _) => Err("missing DST_DEPARTMENT argument".to_string()),
            (_, _, "") => Err("missing name argument".to_string()),
            _ => Ok(Self::MoveEmployee {
                src_department,
                dst_department,
                name,
            }),
        }
    }

    fn new_remove_department(department: String) -> Result<Command, String> {
        match department.as_str() {
            "" => Err("missing DEPARTMENT argument".to_string()),
            _ => Ok(Self::RemoveDepartment { department }),
        }
    }

    fn new_remove_employee(department: String, name: String) -> Result<Command, String> {
        match (department.as_str(), name.as_str()) {
            ("", _) => Err("missing DEPARTMENT argument".to_string()),
            (_, "") => Err("missing NAME argument".to_string()),
            _ => Ok(Self::RemoveEmployee { department, name }),
        }
    }

    fn new_rename_department(
        old_department: String,
        new_department: String,
    ) -> Result<Command, String> {
        match (old_department.as_str(), new_department.as_str()) {
            ("", _) => Err("missing OLD_DEPARTMENT argument".to_string()),
            (_, "") => Err("missing NEW_DEPARTMENT argument".to_string()),
            _ => Ok(Self::RenameDepartment {
                old_department,
                new_department,
            }),
        }
    }

    fn new_rename_employee(
        department: String,
        old_name: String,
        new_name: String,
    ) -> Result<Command, String> {
        match (department.as_str(), old_name.as_str(), new_name.as_str()) {
            ("", _, _) => Err("missing DEPARTMENT argument".to_string()),
            (_, "", _) => Err("missing OLD_NAME argument".to_string()),
            (_, _, "") => Err("missing NEM_NAME argument".to_string()),
            _ => Ok(Self::RenameEmployee {
                department,
                old_name,
                new_name,
            }),
        }
    }

    fn new(
        cmd: String,
        subcmd: String,
        arg0: String,
        arg1: String,
        arg2: String,
    ) -> Result<Command, String> {
        use Command::*;
        let subcmd_s = subcmd.as_str();
        match cmd.as_str() {
            "add" => match subcmd_s {
                "department" => Self::new_add_department(arg0),
                "employee" => Self::new_add_employwee(arg0, arg1),
                _ => Err(format!("invalid 'add' sub command: '{subcmd}'")),
            },
            "exit" => Ok(Exit),
            "list" => match subcmd_s {
                "all" => Ok(ListAll),
                "department" => Self::new_list_department(arg0),
                _ => Err(format!("invalid 'list' sub command: '{subcmd}'")),
            },
            "menu" => Ok(Menu),
            "move" => match subcmd_s {
                "employee" => Self::new_move_employee(arg0, arg1, arg2),
                _ => Err(format!("invalid 'move' sub command: '{subcmd}'")),
            },
            "remove" => match subcmd_s {
                "department" => Self::new_remove_department(arg0),
                "employee" => Self::new_remove_employee(arg0, arg1),
                _ => Err(format!("invalid 'remove' sub command: '{subcmd}'")),
            },
            "rename" => match subcmd_s {
                "department" => Self::new_rename_department(arg0, arg1),
                "employee" => Self::new_rename_employee(arg0, arg1, arg2),
                _ => Err(format!("invalid 'rename' sub command: '{subcmd}'")),
            },
            _ => Err(format!("invalid command: '{cmd}'")),
        }
    }

    const NUM_ARGS: usize = 5;

    fn from_str(input: &str) -> Result<Command, String> {
        let mut words = input.split_whitespace().take(Self::NUM_ARGS);
        let args: Vec<String> = (0..Self::NUM_ARGS)
            .map(|_| match words.next() {
                Some(s) => String::from(s),
                None => String::new(),
            })
            .collect();
        let [cmd, subcmd, arg0, arg1, arg2] = <[String; Self::NUM_ARGS]>::try_from(args).unwrap();
        Self::new(cmd, subcmd, arg0, arg1, arg2)
    }

    const MENU: &'static [&'static str] = &[
        "add department DEPARTMENT",
        "add employee DEPARTMENT NAME ",
        "exit",
        "list all",
        "list department DEPARTMENT",
        "menu",
        "move employee SRC_DEPARTMENT DST_DEPARTMENT NAME",
        "remove department DEPARTMENT",
        "remove employee DEPARTMENT NAME",
        "rename department OLD_DEPARTMENT NEW_DEPARTMENT",
        "rename employee DEPARTMENT OLD_NAME NEW_NAME",
    ];

    fn print_menu(stdout: &Stdout) -> Result<(), String> {
        let mut stdout = stdout.lock();
        writeln!(&mut stdout, "* Available Commands").unwrap();

        for line in Self::MENU {
            writeln!(&mut stdout, "- {line}").unwrap();
        }

        stdout.flush().unwrap();
        Ok(())
    }
}

#[derive(Debug)]
struct DepartmentToEmployeesMap {
    map: BTreeMap<String, BTreeSet<String>>,
}

impl DepartmentToEmployeesMap {
    fn new() -> DepartmentToEmployeesMap {
        DepartmentToEmployeesMap {
            map: BTreeMap::new(),
        }
    }

    fn get_employees(&self, department: &String) -> Result<&BTreeSet<String>, String> {
        match self.map.get(department) {
            Some(employees) => Ok(employees),
            None => Err(format!("no such department: '{department}'")),
        }
    }

    fn get_employees_mut(&mut self, department: &String) -> Result<&mut BTreeSet<String>, String> {
        match self.map.get_mut(department) {
            Some(employees) => Ok(employees),
            None => Err(format!("no such department: '{department}'")),
        }
    }

    fn add_department(&mut self, department: String) -> Result<(), String> {
        if self.map.contains_key(&department) {
            return Err(format!("department already exists: '{department}'"));
        }
        self.map.insert(department, BTreeSet::new());
        Ok(())
    }

    fn add_employee(&mut self, department: String, name: String) -> Result<(), String> {
        let employees = self.get_employees_mut(&department)?;

        if employees.contains(&name) {
            return Err(format!(
                "employee '{name}' already exists in department '{department}'"
            ));
        }

        employees.insert(name);
        Ok(())
    }

    fn list_all(&self, stdout: &Stdout) -> Result<(), String> {
        for department in self.map.keys() {
            self.list_department(department, stdout)?;
        }
        Ok(())
    }

    fn list_department(&self, department: &String, stdout: &Stdout) -> Result<(), String> {
        let mut stdout = stdout.lock();

        writeln!(&mut stdout, "* {department}").unwrap();

        for employee in self.get_employees(department)? {
            writeln!(&mut stdout, "- {employee}").unwrap();
        }

        stdout.flush().unwrap();
        Ok(())
    }

    fn move_employee(
        &mut self,
        src_department: String,
        dst_department: String,
        name: String,
    ) -> Result<(), String> {
        let src_employees = self.get_employees(&src_department)?;
        let dst_employees = self.get_employees(&dst_department)?;

        if !src_employees.contains(&name) {
            return Err(format!(
                "employee '{name}' is not a member of department '{src_department}'"
            ));
        }

        if dst_employees.contains(&name) {
            return Err(format!(
                "employee '{name}' is already a member of department '{dst_department}'"
            ));
        }

        let src_employees = self.get_employees_mut(&src_department)?;
        src_employees.remove(&name);
        self.add_employee(dst_department, name)?;
        Ok(())
    }

    fn remove_department(&mut self, department: String) -> Result<(), String> {
        if !self.map.contains_key(&department) {
            return Err(format!("no such department: '{department}'"));
        }

        if !self.get_employees(&department)?.is_empty() {
            return Err(format!("department '{department}' is not empty"));
        }

        self.map.remove(&department);
        Ok(())
    }

    fn remove_employee(&mut self, department: String, name: String) -> Result<(), String> {
        let employees = self.get_employees_mut(&department)?;

        if !employees.contains(&name) {
            return Err(format!(
                "employee '{name}' is not a member of department '{department}'"
            ));
        }

        employees.remove(&name);
        Ok(())
    }

    fn rename_department(
        &mut self,
        old_department: String,
        new_department: String,
    ) -> Result<(), String> {
        // -- Check that the destination name doesn't exist.
        if self.map.contains_key(&new_department) {
            return Err(format!("department already exists: '{new_department}'"));
        }

        // -- Remove the old department key saving the value.
        let employees = match self.map.remove(&old_department) {
            Some(employees) => employees,
            None => return Err(format!("no such department: '{old_department}'")),
        };

        // -- Reinsert with the new department as key.
        self.map.insert(new_department, employees);
        Ok(())
    }

    fn rename_employee(
        &mut self,
        department: String,
        old_name: String,
        new_name: String,
    ) -> Result<(), String> {
        let employees = self.get_employees_mut(&department)?;

        if !employees.contains(&old_name) {
            return Err(format!(
                "employee '{old_name}' is not a member of department '{department}'"
            ));
        }

        if employees.contains(&new_name) {
            return Err(format!(
                "an employee named '{new_name}' is already a member of department '{department}'"
            ));
        }

        employees.remove(&old_name);
        employees.insert(new_name);
        Ok(())
    }

    fn run(&mut self, cmd: Command, stdout: &Stdout) -> Result<(), String> {
        match cmd {
            Command::AddDepartment { department } => self.add_department(department),
            Command::AddEmployee { department, name } => self.add_employee(department, name),
            Command::Exit => exit(0),
            Command::ListAll => self.list_all(stdout),
            Command::ListDepartment { department } => self.list_department(&department, stdout),
            Command::Menu => Command::print_menu(stdout),
            Command::MoveEmployee {
                src_department,
                dst_department,
                name,
            } => self.move_employee(src_department, dst_department, name),
            Command::RemoveDepartment { department } => self.remove_department(department),
            Command::RemoveEmployee { department, name } => self.remove_employee(department, name),
            Command::RenameDepartment {
                old_department,
                new_department,
            } => self.rename_department(old_department, new_department),
            Command::RenameEmployee {
                department,
                old_name,
                new_name,
            } => self.rename_employee(department, old_name, new_name),
        }
    }
}

fn get_input(stdin: &Stdin, stdout: &Stdout) -> String {
    let mut stdout = stdout.lock();
    write!(&mut stdout, "> ").unwrap();
    stdout.flush().unwrap();

    let mut stdin = stdin.lock();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut department_to_employees = DepartmentToEmployeesMap::new();

    Command::print_menu(&stdout).unwrap();

    loop {
        // println!("DEBUG: {department_to_employees:?}");
        let input = get_input(&stdin, &stdout);
        let result = (|| {
            let cmd = Command::from_str(&input)?;
            department_to_employees.run(cmd, &stdout)
        })();
        match result {
            Ok(()) => (),
            Err(msg) => {
                let mut stdout = stdout.lock();
                writeln!(&mut stdout, "error: {msg}").unwrap();
            }
        }
    }
}
