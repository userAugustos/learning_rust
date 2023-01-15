struct Character {
    name: &str,
    age: i32,
    health: i32 = 100,
    specialPower: &str,
    charge: i32 = 0,
    weapon: &str,
    attack: &str
}

const WARRIOR: Character = Character {
    weapon: "Shotgun",
    name: "Ayoa",
    age: 32,
    specialPower: "Dash Double Shot",
    attack: "Shot"
}

const MAGE: Character = Character {
    name: "Iandara",
    age: 23,
    specialPower: "Rubrum Giant",
    weapon: "Hands Stifly in Roots",
    attack: "wood punch"
}

const ASSASSIN: Character = Character {
    name: "Eun-ji",
    age: 25,
    specialPower: "Sarinja Phi",
    weapon: "Double Scythes",
    attack: "surface cutting"
}

pub const characters: [Character] = [WARRIOR, MAGE, ASSASSIN]