use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        return Err(format!("usage: {} <input>", args[0]));
    }

    let file: File = match File::open(Path::new(&args[1])) {
        Err(e) => {
            return Err(format!("failed to open the file, err - {}", e));
        }
        Ok(f) => f,
    };

    // 1. Read the file line by line
    //
    // 2. For each line break that into multiple parts
    // divided by spaces
    //
    // 3. Each part will be a specific operator or operand
    //
    // 4. Parse them into base-16 numbers

    let lines: Vec<String> = match BufReader::new(file).lines().collect() {
        Ok(lines) => lines,
        Err(e) => {
            return Err(format!("cannot read the file due to - {}", e));
        }
    };

    // Parse the tokens
    let mut outputs: Vec<u8> = Vec::new();

    for l in lines {
        for t in l.split(" ").filter(|x| !x.is_empty()) {
            let b = u8::from_str_radix(t, 16).map_err(|e| format!("parse in error: {}", e))?;
            outputs.push(b);
        }
    }

    let mut out = io::stdout().lock();
    out.write_all(&outputs).map_err(|x| format!("{}", x))?;

    Ok(())
}
