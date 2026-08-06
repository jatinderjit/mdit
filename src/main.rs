use std::env::{Args, args};
use std::error::Error;
use std::path::Path;

struct Cli {
    input: String,
    output: String,
}

impl TryFrom<Args> for Cli {
    type Error = Box<dyn Error>;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let mut it = args.into_iter();
        it.next(); // skip first arg (filename)
        let Some(input) = it.next() else {
            return Err("Input is required".into());
        };
        let output = it.next().unwrap_or_else(|| {
            let (base_name, _) = input.rsplit_once('.').unwrap_or((&input, ""));
            format!("{base_name}.md")
        });
        Ok(Self { input, output })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = args().try_into()?;
    let path = Path::new(&cli.input);
    if !path.is_file() {
        return Err("please enter a valid file path".into());
    }

    let md = anydoc::to_markdown(path)?;
    std::fs::write(Path::new(&cli.output), md)?;

    println!("Saved to {}", cli.output);
    Ok(())
}
