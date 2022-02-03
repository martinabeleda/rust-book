pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.average
    }
    
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_list() {
        let mut ac = AveragedCollection::new();
        
        ac.add(0);
        assert_eq!(ac.average(), 0.0);

        ac.add(1);
        assert_eq!(ac.average(), 0.5);

        ac.add(2);
        assert_eq!(ac.average(), 1.0);
    }

    #[test]
    fn remove_some_values() {
        let mut ac = AveragedCollection::new();
        ac.add(1);
        assert_eq!(ac.average(), 1.0);
        ac.add(2);
        assert_eq!(ac.average(), 1.5);

        // Now, remove some values
        ac.remove();
        assert_eq!(ac.average(), 1.0);
    }

}