
pub struct Config {}

impl TryFrom<&str> for Config {
    type Error = ();
    fn try_from(directory : &str) -> Result<Self,Self::Error> {
        Err(())
    }
}