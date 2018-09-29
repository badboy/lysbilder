use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;

const SLIDES: &str = "slides.md";
const JUMP_CODE: &str = r#"
pub trait Jump {
    type Previous;
    type Next;
}

"#;

fn make_slide_type(slide: Option<usize>) -> String {
    match slide {
        None => format!("()"),
        Some(i) => format!("Slide{}", i),
    }
}

fn make_doc_comment(content: &str) -> String {
    let content = content
        .lines()
        .enumerate()
        .skip_while(|(idx, line)| *idx == 0 && line.is_empty())
        .map(|(_, line)| format!("/// {}", line))
        .collect::<Vec<_>>();
    content.join("\n")
}

fn write_slide_code<W: Write>(
    output: &mut W,
    no: usize,
    content: &str,
    prev: Option<usize>,
    next: Option<usize>,
) -> Result<(), io::Error> {
    let prev = make_slide_type(prev);
    let next = make_slide_type(next);
    let doc_comment = make_doc_comment(content);
    writeln!(
        output,
        r#"
{}
pub struct Slide{};
impl Jump for Slide{} {{
    type Previous = {};
    type Next = {};
}}
"#,
        doc_comment, no, no, prev, next
    );
    Ok(())
}

fn main() -> Result<(), Box<std::error::Error + 'static>> {
    let cwd = env::current_dir()?;

    let slides_file = cwd.join(SLIDES);
    let output_file = cwd.join("src").join("lib.rs");

    let slide_data = fs::read_to_string(slides_file)?;
    let slides = slide_data.split("---\n").collect::<Vec<_>>();
    let slides = if slides[0].is_empty() {
        &slides[1..]
    } else {
        &slides[..]
    };
    let total_slides = slides.len();

    let slides = slides.iter().enumerate().map(|(idx, slide)| {
        let prev = if idx == 0 { None } else { Some(idx) };
        let next = if idx + 1 == total_slides {
            None
        } else {
            Some(idx + 2)
        };
        (idx + 1, slide, prev, next)
    });

    let mut output = File::create(output_file)?;
    writeln!(output, "{}", JUMP_CODE);
    for slide in slides {
        write_slide_code(&mut output, slide.0, slide.1, slide.2, slide.3)?;
    }

    Command::new("rustdoc")
        .arg("--html-in-header").arg("lysbilder.html")
        .arg("src/lib.rs")
        .status()?;

    Ok(())
}
