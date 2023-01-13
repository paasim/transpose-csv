use std::io::{BufRead, Error, Lines, Write};

fn read_first_line<R: BufRead>(lines: &mut Lines<R>) -> Result<Vec<String>, Error> {
    let first_ln = lines.next().unwrap_or(Ok(String::new()))?;
    Ok(first_ln.split(',').map(|rec| rec.to_string()).collect())
}

fn add_line(mat: &mut Vec<String>, ln: String) {
    mat.iter_mut().zip(ln.split(',')).for_each(|(row, rec)| {
        row.push(',');
        *row += rec;
    })
}

fn read_lines<R: BufRead>(lines: &mut Lines<R>, mat: &mut Vec<String>) -> Result<(), Error> {
    lines.try_for_each(|ln| Ok(add_line(mat, ln?)))
}

fn write_lines<W: Write>(hndl: &mut W, mat: &mut Vec<String>) -> Result<(), Error> {
    mat.into_iter()
        .try_for_each(|ln| Ok(writeln!(hndl, "{}", ln)?))
}

fn read_and_write() -> Result<(), Error> {
    let lines = &mut std::io::stdin().lock().lines();
    let mat = &mut read_first_line(lines)?;

    read_lines(lines, mat)?;
    write_lines(&mut std::io::stdout(), mat)?;
    Ok(())
}

fn main() {
    read_and_write().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
}
