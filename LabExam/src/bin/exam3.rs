trait Diet {
    fn food(&self)->String;
}

trait Habitat{
    fn environment(&self)->String;
}

struct Duck;

impl Diet for Duck{
    fn food(&self)->String{
        "plants and small fishes".to_string()
    }
}

impl Habitat for Duck{
    fn environment(&self)->String{
        "wetlands and ponds ".to_string()
    }
}

struct Chicken;

impl Diet for Chicken{
    fn food(&self)->String{
        "seeds and insects".to_string()
    }
}

impl Habitat for Chicken{
    fn environment(&self)->String{
        "farms and backyards".to_string()
    }
}

fn describe_animal<T:Diet+Habitat>(animal:&T){
    println!("This animal eats {} and lives in a {} environment.", animal.food(), animal.environment());
}

fn main(){
    let duck=Duck;
    describe_animal(&duck);
    let chicken=Chicken;
    describe_animal(&chicken);
}