fn soma(a: u32, b: u32) -> u32 {
    a + b
}

struct Person {
    name: String,
    age: u32
}

impl Person {
    fn introducing_my_self(&self){
        println!("Meu nome é {} e minha idade é {}", self.name, self.age)
    }
}

#[allow(unused_variables)]
fn main() {
    let person = Person{name: String::from("Cloves"), age: 32};
    //println!("Meu nome é {} e minha idade é {}", person.name, person.age);
    person.introducing_my_self();
    let a: u32 = 2;
    let b: u32 = 3;
    let res: u32 = soma(a, b);
    println!("{a} + {b} = {res}");
}