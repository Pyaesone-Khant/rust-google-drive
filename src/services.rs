use colored::Colorize;

#[derive(Debug)]
pub enum SERVICES {
    DisplayFiles,
    UploadFile,
}

pub fn ask_user_for_service() -> SERVICES {
    let mut input = String::new();
    let mut service: Option<SERVICES> = None;

    while service.is_none() {
        input.clear();
        print!(
            "{}",
            "\nPlease select desired service by entering the number: 😉"
                .green()
                .bold()
        );
        println!(
            "
|-----------------------------------|
|                                   |
|       1️⃣. {}            |
|       2️⃣. {}          |
|       3️⃣. {}             |
|                                   |
|-----------------------------------|        
    ",
            "Display Files".yellow().bold(),
            "Upload to Drive".purple().bold(),
            "Quit Program".red().bold()
        );

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read service option input!");

        match input.trim().parse::<i32>() {
            Ok(num) => match num {
                1 => {
                    service = Some(SERVICES::DisplayFiles);
                }
                2 => {
                    service = Some(SERVICES::UploadFile);
                }
                3 => std::process::exit(0),
                _ => println!("❌. Service not available! Please select the available service. 😭"),
            },
            Err(_err) => {
                println!("❌. Invalid input received, please select the service again. 😭")
            }
        }
    }

    service.unwrap()
}
