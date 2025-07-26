use std::time::Duration;

use dotenv::dotenv;
use include_directory::{Dir, DirEntry, include_directory};
use tokio::time::sleep;

static PROJECT_DIR: Dir<'_> = include_directory!("../out_md");

const QUESTIONS: &str = r#"

- Parteien: Welches sind die beteiligten Parteien?

- Sachverhalt: Wer hat was wann getan?

- Parteibegehren: Wer will was von wem woraus?

- Streitpunkt: Was war streitig / das Problem?

- Position der Vorinstanzen: Wie haben die Vorinstanzen entschieden?

- Schlussfolgerung des Bundesgerichts: Wie hat das Bundesgericht entschieden?

"#;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut bges: Vec<BGE> = vec![];

    for entry in PROJECT_DIR.entries().into_iter() {
        println!("Sleeping to not stress llm");
        sleep(Duration::from_secs(5)).await;
        let bge = handle_entry(&entry).await;
        bges.push(bge);
    }

    for bge in bges {
        println!("Writing {}", &bge.title);
        write_bge_to_file(&bge);
    }
}

pub async fn handle_entry(entry: &DirEntry<'_>) -> BGE {
    let file_name = entry.as_file().unwrap().path().display().to_string();
    let content = entry.as_file().unwrap().contents_utf8().unwrap();

    let mut bge = BGE::new(file_name.clone(), content.to_string());
    println!("Asking llm for {}", &file_name);

    ask_llm(&mut bge).await;

    println!("Done with {}", &file_name);

    bge
}

pub struct BGE {
    title: String,
    content: String,
    answer: Option<String>,
}

impl BGE {
    pub fn new(title: String, content: String) -> Self {
        BGE {
            title,
            content,
            answer: None,
        }
    }

    pub fn ai_prompt(&self) -> String {
        format!(
            "
            Du gibts Antworten zu folgenend Fragen:

            {}

            Benutze diesen BGE als Kontext:

            {}: {}",
            QUESTIONS, self.title, self.content
        )
    }

    pub fn add_answer(&mut self, answer: String) {
        self.answer = Some(answer);
    }

    pub fn answer(&self) -> Option<&str> {
        self.answer.as_deref()
    }
}

pub async fn ask_llm(bge: &mut BGE) {
    let answer = gemeni(bge).await;
    bge.add_answer(answer);
}

pub async fn gemeni(bge: &BGE) -> String {
    gemini_rs::chat("gemini-2.0-flash")
        .send_message(&bge.ai_prompt())
        .await
        .unwrap()
        .candidates
        .get(0)
        .unwrap()
        .content
        .parts
        .iter()
        .map(|p| p.text.clone().unwrap().to_string())
        .into_iter()
        .collect::<String>()
}

pub fn write_bge_to_file(bge: &BGE) {
    let file_name = format!("answers/{}.md", bge.title);
    let content = format!(
        "\n{}\n\n\n## Antworten\n{}\n",
        bge.content,
        bge.answer().unwrap_or("No answer")
    );

    std::fs::write(file_name, content).unwrap();
}
