use regex::Regex;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::process;
use std::thread;

use rocket_contrib::serve::StaticFiles;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

fn main() {
    refresh_html();
    let folder = env::args().nth(2).expect("Folder argument is required");
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    // Create a thread to run the Rocket server
    thread::spawn(move || {
        rocket::ignite()
            .mount("/", StaticFiles::from(&folder))
            .launch();
    });

    // Create a thread to watch the path
    thread::spawn(move || {
        println!("watching {}", path);
        if let Err(e) = watch(path) {
            println!("error: {:?}", e);
        }
    });

    // Wait for both threads to finish
    thread::sleep(std::time::Duration::from_secs(u64::MAX));
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("changed: {:?}", event);
                refresh_html()
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn refresh_html() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("Please provide the directory containing Markdown files and the directory for HTML output");
        return;
    }

    let markdown_dir = &args[1];
    let html_dir = &args[2];

    process_directory(Path::new(markdown_dir), Path::new(html_dir));
}

fn process_directory(markdown_dir: &Path, html_dir: &Path) {
    if let Ok(entries) = fs::read_dir(markdown_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();
                let file_path = entry.path();

                if file_type.is_dir() {
                    let sub_dir_name = file_path.file_name().unwrap_or_default();
                    let sub_html_dir = html_dir.join(sub_dir_name);
                    fs::create_dir_all(&sub_html_dir).unwrap_or_else(|error| {
                        eprintln!("Error creating directory: {:?}", error);
                    });

                    process_directory(&file_path, &sub_html_dir);
                } else if file_type.is_file() && file_path.extension().unwrap_or_default() == "md" {
                    let file_name = file_path.file_stem().unwrap_or_default();
                    let output_path = html_dir.join(file_name).with_extension("html");

                    let mut file = match File::open(&file_path) {
                        Ok(file) => file,
                        Err(error) => {
                            eprintln!("Error opening file: {:?}", error);
                            continue;
                        }
                    };

                    let mut contents = String::new();
                    if let Err(error) = file.read_to_string(&mut contents) {
                        eprintln!("Error reading file: {:?}", error);
                        continue;
                    }

                    let html_contents = wrap_in_html(contents);

                    if let Err(error) = fs::write(&output_path, &html_contents) {
                        eprintln!("Error writing file: {:?}", error);
                        continue;
                    }

                    println!("Generated HTML: {}", output_path.display());
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {:?}", markdown_dir);
    }
}

// Replace text using regular expressions and a format string
fn replace_text(pattern: &str, format_string: &str, transformed_section: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    regex
        .replace_all(transformed_section, |caps: &regex::Captures| {
            // Replace text using regex
            format_string.replace("{}", &caps[1])
        })
        .to_string()
}

// Wrap the generated into a HTML document.
fn wrap_in_html(html: String) -> String {
    let args: Vec<_> = env::args().collect();

    let filename = args[1].to_string();

    let mut file = match File::open(Path::new(&filename).join("template.html")) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening template.html file: {:?}", error);
            process::exit(1);
        }
    };

    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        eprintln!("Error reading template.html file: {:?}", error);
        process::exit(1);
    }

    return contents.replace("{{{}}}", &md_to_html(html));
}

// Convert Markdown to HTML
fn md_to_html(md: String) -> String {
    let sections = md.split("```").enumerate().collect::<Vec<(usize, &str)>>();
    let mut output = String::new();

    for (index, section) in sections {
        let is_inside_code_block = index % 2 == 1;
        if is_inside_code_block {
            let mut transformed_section = section.to_string();

            // Replace paragraphs
            transformed_section = replace_text(r"(?m)^(.*?)$", "<p>{}</p>", &transformed_section);

            output.push_str(&format!(
                "<pre><code>{}</code></pre>",
                transformed_section.replace("<p></p>", "").replace("\n", "")
            ));
        } else {
            let mut transformed_section = String::new();

            // Replace paragraphs
            let lines: Vec<&str> = section
                .lines()
                .map(|line| line.trim())
                .collect();

            for line in lines {
                if line.starts_with("#") {
                    transformed_section += &format!("{}\n", line);
                } else if line == "" {
                    transformed_section += "<br>\n"
                } else {
                    transformed_section += &format!("<p>{}</p>\n", line);
                }
            }

            // Replace headers
            let header_pattern = Regex::new(r"(?m)^(#{0,6})\s+(.*)$").unwrap();
            transformed_section = header_pattern
                .replace_all(&transformed_section, |caps: &regex::Captures| {
                    let level = caps[1].len();

                    format!("<h{0}>{1}</h{0}>", level, &caps[2])
                })
                .to_string();

            // Replace links
            let link_pattern = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
            transformed_section = link_pattern
                .replace_all(&transformed_section, |caps: &regex::Captures| {
                    format!("<a href=\"{}\">{}</a>", &caps[2], &caps[1])
                })
                .to_string();

            // Replace bold text
            transformed_section = replace_text(
                r"\*\*(.*?)\*\*",
                "<strong>{}</strong>",
                &transformed_section,
            );

            // Replace italic text
            transformed_section = replace_text(r"\*(.*?)\*", "<i>{}</i>", &transformed_section);

            // Replace underline text
            transformed_section = replace_text(r"__(.*?)__", "<u>{}</u>", &transformed_section);

            output.push_str(&transformed_section.replace("<p></p>", "").replace("\n", ""));
        }
    }
    return output;
}