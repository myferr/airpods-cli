mod airpods;
mod ui;

use clap::Parser;
use colored::*;
use airpods::*;
use ui::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Disconnect AirPods
    #[arg(long)]
    disconnect: bool,

    /// Disable Unicode icons
    #[arg(long)]
    no_unicode: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.disconnect {
        match disconnect_airpods().await {
            Ok(msg) => println!("{}", msg.green()),
            Err(e) => eprintln!("{} {}", "Failed to disconnect:".red(), e),
        }
        return;
    }

    match get_airpods_info().await {
        Ok(info) => {
            println!(
                "\n{} {} Battery Levels",
                if args.no_unicode { "" } else { "🔋" },
                info.device_name.bold()
            );
            println!("┌───────────────────────────────────────────────┐");

            let batteries = [
                ("Left", info.left_battery),
                ("Right", info.right_battery),
            ];

            let available = batteries
                .iter()
                .filter_map(|(label, opt)| opt.map(|v| (*label, v)))
                .collect::<Vec<_>>();

            if available.is_empty() {
                println!("{}", "❌ Battery info not available.".red());
            } else {
                for (label, val) in available {
                    render_bar(label, val, args.no_unicode);
                }
            }

            println!("└───────────────────────────────────────────────┘\n");
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red(), e);
        }
    }
}
