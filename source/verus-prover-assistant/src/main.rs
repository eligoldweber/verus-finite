use clap::Parser;
use prettyplease_verus::unparse;
use std::fs;
use syn_verus::visit::{visit_file, Visit};
use syn_verus::{File, ItemMacro};
use verusfmt::{self, RunOptions};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    entrypoint: String,
}

struct VerusMacroVisitor<'ast> {
    verus_macros: Vec<&'ast ItemMacro>,
}

impl<'ast> Visit<'ast> for VerusMacroVisitor<'ast> {
    fn visit_item_macro(&mut self, mac: &'ast ItemMacro) {
        if mac.mac.path.is_ident("verus") {
            self.verus_macros.push(mac);
        }
        syn_verus::visit::visit_item_macro(self, mac);
    }
}

fn main() {
    let args = Args::parse();

    let file_content = fs::read_to_string(&args.entrypoint).expect("Failed to read the file");

    let syntax_tree: File =
        syn_verus::parse_file(&file_content).expect("Failed to parse Rust file");

    let mut visitor = VerusMacroVisitor { verus_macros: Vec::new() };
    visit_file(&mut visitor, &syntax_tree);

    let formatted_code = unparse(&syntax_tree);

    let output_path = "formatted_output.rs";
    fs::write(&output_path, formatted_code).expect("Failed to write to file");

    println!("Prettyplease formatted code written to: {}", output_path);

    let formatted_output = verusfmt::run(
        &fs::read_to_string(output_path).expect("Failed to read the file"), // Read the file contents
        RunOptions {
            file_name: Some(output_path.to_string()), // Specify the file to format
            run_rustfmt: true,                        // Optionally run rustfmt on non-verus parts
            rustfmt_config: Default::default(),
        },
    );

    // Handle the output and print it or report the error.
    match formatted_output {
        Ok(output) => {
            println!("{}", output);
            fs::write(output_path, output).expect("Failed to write the formatted file");
        }
        Err(err) => eprintln!("Failed to format file: {:?}", err), // Print the error if formatting fails
    }

    println!("verusfmt successfully formatted the file.");
}
