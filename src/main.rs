use serde_json::{json, Value};
use simple_user_input::get_input;
use std::thread;

const TITLE_L: &str = r#"                  ___           ___           ___                       ___           ___           ___           ___     
      ___        /  /\         /  /\         /__/\          ___        /  /\         /  /\         /  /\         /__/|    
     /  /\      /  /:/_       /  /::\       |  |::\        /  /\      /  /::\       /  /::\       /  /:/        |  |:|    
    /  /:/     /  /:/ /\     /  /:/\:\      |  |:|:\      /  /:/     /  /:/\:\     /  /:/\:\     /  /:/         |  |:|    
   /  /:/     /  /:/ /:/_   /  /:/~/:/    __|__|:|\:\    /  /:/     /  /:/~/:/    /  /:/~/::\   /  /:/  ___   __|  |:|    
  /  /::\    /__/:/ /:/ /\ /__/:/ /:/___ /__/::::| \:\  /  /::\    /__/:/ /:/___ /__/:/ /:/\:\ /__/:/  /  /\ /__/\_|:|____
 /__/:/\:\   \  \:\/:/ /:/ \  \:\/:::::/ \  \:\~~\__\/ /__/:/\:\   \  \:\/:::::/ \  \:\/:/__\/ \  \:\ /  /:/ \  \:\/:::::/
 \__\/  \:\   \  \::/ /:/   \  \::/~~~~   \  \:\       \__\/  \:\   \  \::/~~~~   \  \::/       \  \:\  /:/   \  \::/~~~~ 
      \  \:\   \  \:\/:/     \  \:\        \  \:\           \  \:\   \  \:\        \  \:\        \  \:\/:/     \  \:\     
       \__\/    \  \::/       \  \:\        \  \:\           \__\/    \  \:\        \  \:\        \  \::/       \  \:\    
                 \__\/         \__\/         \__\/                     \__\/         \__\/         \__\/         \__\/    "#;

fn main() {
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::cursor::Hide).unwrap();
    let (w, h) = crossterm::terminal::size().unwrap();
    let screen_width = w as u16;
    let screen_height = h as u16;

    // get level ID:
    // let level_id: String = get_input("level ID: ");
    let level_id: &str = "showcase3196493566";
    let mut old_leaderstring: String = String::new();

    // print background image
    print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
    for _row in 0..=screen_height {
        println!("{}\r", " ".repeat(screen_width as usize),)
    }

    // print title
    let lines: Vec<&str> = TITLE_L.split("\n").collect();
    let x = screen_width / 2 - (lines[0].len() as u16) / 2;
    let mut y: u16 = 3;
    for line in &lines {
        println!(
            "{esc}[{};{}H{}",
            y,
            screen_width / 2 - (line.len() as u16) / 2,
            line,
            esc = 27 as char
        );
        y += 1;
    }

    y += 1;

    println!(
        "{esc}[{};{}H{:-^3$}",
        y,
        x,
        "",
        lines[0].len() as usize,
        esc = 27 as char
    );
    y += 2;

    loop {
        // get leaderboard
        let leader_string = match reqwest::blocking::get(&format!(
            "http://danielsson.pythonanywhere.com/get_result/{level_id}"
        )) {
            Ok(resp) => resp.text().unwrap(),
            Err(resp) => panic!("Err: {}", resp),
        };
        let mut leader_board: Value = serde_json::from_str(&leader_string).unwrap_or(json!([]));
        let leader_vec = leader_board.as_array_mut().expect("leaderboard error");

        leader_vec.sort_by_key(|val| {
            (val.get("time")
                .expect("leaderboard format wrong")
                .as_f64()
                .expect("leaderboard format wrong")
                * 1000.) as usize
        });
        // print leaderboard
        if old_leaderstring != leader_string {
            print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
            for row in y..=screen_height {
                print!("\x1b[{row};0H{}\r", " ".repeat(screen_width as usize),)
            }

            for (i, result) in leader_vec.iter().enumerate() {
                let mut name = result.get("name").unwrap().as_str().unwrap().to_string();
                let time = result.get("time").unwrap().as_f64().unwrap();
                let loop_threashold = (screen_height - y - 2) as usize;
                let max_width = 25;

                if i.div_euclid(loop_threashold) >= 3 {
                    break;
                }

                if name.len() > max_width {
                    name = name[0..(max_width as usize - 3)].to_string() + "...";
                }

                println!(
                    "{esc}[{};{}H{}",
                    y + (i as u16) % loop_threashold as u16,
                    x + (40 * i.div_euclid(loop_threashold)) as u16 + 9,
                    format!("{}. {} - {:.2}s", i + 1, name, time,),
                    esc = 27 as char
                );
            }
        }

        old_leaderstring = leader_string;
        thread::sleep_ms(1000);
    }
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }
}
