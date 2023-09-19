//use probabst::set::Set;
use probabst::set::*;
// tests for set

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_creation() {
    }
    #[test]
    fn test_set_intersection() {
        // create sets
        let mut set1 = Set::empty();
        set1.add(1);
        set1.add(2);
        let mut set2 = Set::empty();
        set2.add(2);
        set2.add(3);

        let set3 = set1.intersection(&set2);
        //assert_eq!(set1.intersection(&set2), set3);
    }

}