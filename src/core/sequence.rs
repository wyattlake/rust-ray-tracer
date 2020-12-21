//The Sequence struct is used to generate offset values for AreaLights
#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub contents: Vec<f32>,
    pub current_index: usize,
}

impl Sequence {
    //Creates a new Sequence
    pub fn new(list: Vec<f32>) -> Sequence {
        Sequence {
            contents: list,
            current_index: 0,
        }
    }

    //Creates a blank sequence
    pub fn blank() -> Sequence {
        Sequence {
            contents: vec![0.5],
            current_index: 0,
        }
    }

    //Moves to the next reference
    pub fn next(&mut self) -> f32 {
        let index = self.current_index.clone();
        if self.current_index < self.contents.len() - 1 {
            self.current_index += 1;
            self.contents[index]
        } else {
            self.current_index = 0;
            self.contents[index]
        }
    }

    //Gets the current reference
    pub fn get(&self) -> f32 {
        self.contents[self.current_index]
    }
}
