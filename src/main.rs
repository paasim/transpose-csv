use error::Res;
use std::io;
use std::process;
use std::io::{BufRead, Lines, Write};

mod error;

fn read_first_line<R: BufRead>(lines: &mut Lines<R>) -> Res<Vec<String>> {
    let mut mat = Vec::<String>::new();
    let first_ln = lines.next().ok_or("No lines")?;
    first_ln?.split(',').for_each(|rec| {
        mat.push(rec.to_string());
    });
    Ok(mat)
}

fn add_line(ln: String, mat: &mut Vec<String>) {
    mat.iter_mut()
        .zip(&mut ln.split(','))
        .for_each(|(row, rec)| {
            row.push(',');
            row.push_str(rec);
        })
}

fn read_lines<R: BufRead>(hndl: &mut Lines<R>, mat: &mut Vec<String>) -> Res<()> {
    hndl.try_for_each(|ln| Ok(add_line(ln?, mat)))
}

fn write_lines<W: Write>(hndl: &mut W, mat: Vec<String>) -> Res<()> {
    mat.into_iter()
        .try_for_each(|ln| Ok(writeln!(hndl, "{}", ln)?))
}

fn read_and_write(stdin: &io::Stdin, stdout: &mut io::Stdout) -> Res<()> {
    let mut lines = stdin.lock().lines();

    let mut mat = read_first_line(&mut lines)?;
    read_lines(&mut lines, &mut mat)?;
    write_lines(stdout, mat)?;
    Ok(())
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    read_and_write(&stdin, &mut stdout).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
}
