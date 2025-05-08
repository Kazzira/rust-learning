pub trait PhraseComponent {
    fn to_string(&self);
    fn next_component(&self) -> &dyn PhraseComponent;
}

type Age = i16;

pub enum Gender {
    Male,
    Female,
    Other,
}

pub enum Article {
    The,
    A,
    Number(u64),
}

pub struct AnimalName {
    pub name: String,
}

pub enum AnimalPreferredAddressing {
    Name(AnimalName, String, Gender),
    Pronoun(Gender),
    AgeSex(Article, AnimalName, Age, Gender),
}

pub enum Actor {
    Person(PersonPreferredAddressing),
    Animal(AnimalPreferredAddressing),
}

pub enum Object {
    Actor(Actor),
    Item(Article, String),
}

pub enum PersonPreferredAddressing {
    Name(String, Gender),
    Pronoun(Gender),
    AgeSex(Article, Age, Gender),
}

impl Gender {
    pub fn to_child_string(&self) -> String {
        match self {
            Gender::Male => "boy".to_owned(),
            Gender::Female => "girl".to_owned(),
            Gender::Other => "child".to_owned(),
        }
    }

    pub fn to_informal_adult_string(&self) -> String {
        match self {
            Gender::Male => "man".to_owned(),
            Gender::Female => "woman".to_owned(),
            Gender::Other => "person".to_owned(),
        }
    }

    pub fn to_formal_adult_string(&self) -> String {
        match self {
            Gender::Male => "gentleman".to_owned(),
            Gender::Female => "lady".to_owned(),
            Gender::Other => "gentleperson".to_owned(),
        }
    }

    pub fn to_pronoun_string(&self) -> String {
        match self {
            Gender::Male => "he".to_owned(),
            Gender::Female => "she".to_owned(),
            Gender::Other => "them".to_owned(),
        }
    }
}
