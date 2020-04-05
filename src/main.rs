trait Born {
    fn born(&mut self) -> bool;
}

trait Die {
    fn die(&mut self) -> bool;
}

trait Animal: Born + Die {
    fn is_alive(&self) -> bool;
}

struct Dog {
    alive: Option<bool>,
    name: String,
}

impl Animal for Dog {
    fn is_alive(&self) -> bool {
        match self.alive {
            Some(true) => {
                println!("dog[{}] is still alive!", self.name);
                true
            }
            Some(false) => {
                println!("dog[{}] is dead!", self.name);
                false
            }
            None => {
                println!("dog[{}] have not born yet!", self.name);
                false
            }
        }
    }
}

impl Dog {
    fn new(name: String) -> Dog {
        Dog { name, alive: None }
    }
}

impl Born for Dog {
    fn born(&mut self) -> bool {
        if let Some(s) = self.alive {
            if s {
                println!("dog[{}] have already borned!", self.name);
                return false;
            }
            println!("dog[{}] have already died!", self.name);
            return false;
        }
        self.alive = Some(true);
        println!("dog[{}] borned!", self.name);
        return true;
    }
}

impl Die for Dog {
    fn die(&mut self) -> bool {
        if let Some(s) = self.alive {
            if s {
                self.alive = Some(false);
                println!("dog[{}] died!", self.name);
                return true;
            }
            println!("dog[{}] have already dead", self.name);
            return false;
        }
        println!("dog [{}]have not born yet!", self.name);
        return false;
    }
}

struct Cat {
    alive: Option<bool>,
    name: String,
}

impl Animal for Cat {
    fn is_alive(&self) -> bool {
        match self.alive {
            Some(true) => {
                println!("cat[{}] is still alive!", self.name);
                true
            }
            Some(false) => {
                println!("cat[{}] is dead!", self.name);
                false
            }
            None => {
                println!("cat[{}] have not born yet!", self.name);
                false
            }
        }
    }
}

impl Cat {
    fn new(name: String) -> Cat {
        Cat { name, alive: None }
    }
}

impl Born for Cat {
    fn born(&mut self) -> bool {
        if let Some(s) = self.alive {
            if s {
                println!("cat[{}] have already borned!", self.name);
                return false;
            }
            println!("cat[{}] have already died!", self.name);
            return false;
        }
        self.alive = Some(true);
        println!("cat[{}] borned!", self.name);
        return true;
    }
}

impl Die for Cat {
    fn die(&mut self) -> bool {
        if let Some(s) = self.alive {
            if s {
                self.alive = Some(false);
                println!("cat[{}] died!", self.name);
                return true;
            }
            println!("cat[{}] have already dead", self.name);
            return false;
        }
        println!("cat[{}] have not born yet!", self.name);
        return false;
    }
}

fn main() {
    let mut ani_vec: Vec<Box<dyn Animal>> = Vec::new();

    ani_vec.push(Box::new(Dog::new(String::from("taro"))));
    ani_vec.push(Box::new(Dog::new(String::from("jiro"))));
    ani_vec.push(Box::new(Dog::new(String::from("saburo"))));
    ani_vec.push(Box::new(Cat::new(String::from("nyan"))));
    ani_vec.push(Box::new(Cat::new(String::from("neko"))));
    ani_vec.push(Box::new(Cat::new(String::from("nuko"))));

    for a in ani_vec.iter_mut() {
        a.is_alive();
        a.born();
        a.is_alive();
        a.die();
        a.is_alive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dog_cycle_test() {
        {
            println!("nomal cycle");
            let mut dog = Dog::new(String::from("taro"));
            assert_eq!(dog.is_alive(), false);
            assert_eq!(dog.born(), true);
            assert_eq!(dog.is_alive(), true);
            assert_eq!(dog.die(), true);
            assert_eq!(dog.is_alive(), false);
        }
        {
            println!("die before born");
            let mut dog = Dog::new(String::from("taro"));
            assert_eq!(dog.is_alive(), false);
            assert_eq!(dog.die(), false);
            assert_eq!(dog.is_alive(), false);
        }
        {
            println!("born twice");
            let mut dog = Dog::new(String::from("taro"));
            assert_eq!(dog.is_alive(), false);
            assert_eq!(dog.born(), true);
            assert_eq!(dog.born(), false);
            assert_eq!(dog.is_alive(), true);
        }
        {
            println!("die twice");
            let mut dog = Dog::new(String::from("taro"));
            assert_eq!(dog.is_alive(), false);
            assert_eq!(dog.born(), true);
            assert_eq!(dog.die(), true);
            assert_eq!(dog.die(), false);
            assert_eq!(dog.is_alive(), false);
        }
    }
    #[test]
    fn cat_cycle_test() {
        {
            println!("nomal cycle");
            let mut cat = Cat::new(String::from("taro"));
            assert_eq!(cat.is_alive(), false);
            assert_eq!(cat.born(), true);
            assert_eq!(cat.is_alive(), true);
            assert_eq!(cat.die(), true);
            assert_eq!(cat.is_alive(), false);
        }
        {
            println!("die before born");
            let mut cat = Cat::new(String::from("taro"));
            assert_eq!(cat.is_alive(), false);
            assert_eq!(cat.die(), false);
            assert_eq!(cat.is_alive(), false);
        }
        {
            println!("born twice");
            let mut cat = Cat::new(String::from("taro"));
            assert_eq!(cat.is_alive(), false);
            assert_eq!(cat.born(), true);
            assert_eq!(cat.born(), false);
            assert_eq!(cat.is_alive(), true);
        }
        {
            println!("die twice");
            let mut cat = Cat::new(String::from("taro"));
            assert_eq!(cat.is_alive(), false);
            assert_eq!(cat.born(), true);
            assert_eq!(cat.die(), true);
            assert_eq!(cat.die(), false);
            assert_eq!(cat.is_alive(), false);
        }
    }
}
