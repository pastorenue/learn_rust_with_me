pub trait Growable {
    fn is_growing(&self);
}

#[derive(Debug)]
pub struct PlantInfo {
    pub botanica_name: String,
    pub age: i32,
    pub is_grown: bool,
    pub species: String,
    pub family: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Herb {
    pub info: PlantInfo,
}

#[derive(Debug)]
pub struct Shrub {
    pub info: PlantInfo,
}

#[derive(Debug)]
pub struct Tree {
    pub info: PlantInfo,
}

#[derive(Debug)]
pub struct Climber {
    pub info: PlantInfo,
}

#[derive(Debug)]
pub struct Creeper {
    pub info: PlantInfo,
}

#[derive(Debug)]
pub enum PlantClassification {
    Herb(Herb),
    Shrub(Shrub),
    Tree(Tree),
    Climber(Climber),
    Creeper(Creeper),
}

impl Growable for Herb {
    fn is_growing(&self) {
        println!("I am growing as a {} plant", self.info.botanica_name);
    }
}

impl Growable for Shrub {
    fn is_growing(&self) {
        println!("I am growing as a {} plant", self.info.botanica_name);
    }
}

impl Growable for Tree {
    fn is_growing(&self) {
        println!("I am growing as a {} plant", self.info.botanica_name);
    }
}

impl Growable for Climber {
    fn is_growing(&self) {
        println!("I am growing as a {} plant", self.info.botanica_name);
    }
}

impl Growable for Creeper {
    fn is_growing(&self) {
        println!("I am growing as a {} plant", self.info.botanica_name);
    }
}

impl Growable for PlantClassification {
    fn is_growing(&self) {
        match self {
            PlantClassification::Herb(h) => h.is_growing(),
            PlantClassification::Shrub(s) => s.is_growing(),
            PlantClassification::Tree(t) => t.is_growing(),
            PlantClassification::Climber(c) => c.is_growing(),
            PlantClassification::Creeper(c) => c.is_growing(),
        }
    }
}

pub fn grow(plant: Box<dyn Growable>) {
    plant.is_growing();
}
