enum Day{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}
impl Day{
    fn is_weekend(&self) -> bool{
        match self{
            Day::Saturday | Day::Sunday => true,
            _ => false
        }
    }

    fn is_weekday(&self) -> bool{
        !self.is_weekend()
    }
}
pub(crate) fn run() {
    let day = Day::Monday;
    println!("Monday is weekend? {}", day.is_weekend());
    println!("Monday is week day? {}", day.is_weekday());
}