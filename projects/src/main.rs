type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn table_of_contents(table: &toml::Table) {
    let names = ["Inactive", "Active"];
    let mut projects_by_active = [vec![], vec![]];
    for (href, table) in table {
        let name = table
            .get("name")
            .and_then(|name| name.as_str())
            .unwrap_or(href);
        let is_active = table
            .get("active")
            .and_then(|active| active.as_bool())
            .unwrap_or(false);
        projects_by_active[is_active as usize].push((href, name));
    }

    println!("<dl>");
    for (i, projects) in projects_by_active.iter_mut().enumerate().rev() {
        projects.sort_by(|(_, name_lhs), (_, name_rhs)| name_lhs.cmp(name_rhs));
        println!("<dt><h3>{label}<h3></dt><dd>", label = names[i]);
        println!(
            "{}",
            itertools::join(
                projects
                    .iter()
                    .map(|(href, name)| format!("<a href=\"#{href}\">{name}</a>")),
                ",<br />"
            )
        );
        println!("</dd>");
    }
    println!("</dl>");
}

fn sections(table: &toml::Table) {
    let names = ["Inactive", "Active"];
    let mut projects_by_active = [vec![], vec![]];
    for (href, table) in table {
        let is_active = table
            .get("active")
            .and_then(|active| active.as_bool())
            .unwrap_or(false);
        projects_by_active[is_active as usize].push((href, table));
    }

    for (i, projects) in projects_by_active.iter_mut().enumerate().rev() {
        println!("<section><h2>{label}</h2>", label = names[i]);

        projects.sort_by(|(lhs_href, lhs_table), (rhs_href, rhs_table)| {
            // TODO: Maybe compare by date or add additional ordering parameter in TOML file?
            // It may be not so good to have as the most important (at top) projects that are
            // uninteresting just because they are named lexicographically smaller
            let lhs_name = lhs_table
                .get("name")
                .and_then(|name| name.as_str())
                .unwrap_or(&lhs_href);
            let rhs_name = rhs_table
                .get("name")
                .and_then(|name| name.as_str())
                .unwrap_or(&rhs_href);
            lhs_name.cmp(rhs_name)
        });

        for (href, project) in projects {
            let name = project
                .get("name")
                .and_then(|name| name.as_str())
                .unwrap_or(&href);

            let description = project
                .get("description")
                .and_then(|description| description.as_str())
                .expect("description");

            let started_at = project
                .get("started_at")
                .and_then(|started_at| {
                    started_at
                        .as_datetime()
                        .and_then(|datetime| datetime.date)
                        .map(|date| format!("{date}"))
                })
                .expect("started_at");

            let url = project
                .get("url")
                .and_then(|url| url.as_str())
                .map(|url| {
                    if url.contains("github.com/") {
                        format!("<a href=\"{url}\">GitHub</a>")
                    } else {
                        format!("<a href=\"{url}\">{url}</a>")
                    }
                })
                .unwrap_or("-".to_string());

            println!("<p><h3 id=\"{href}\"><a href=\"#{href}\">{name}</a></h3>");
            println!("<table><tr><th>Started in</th><th>URL</th></tr>");
            println!("<tr><td>{started_at}</td><td>{url}</td></tr></table>");
            println!("{description}");
            println!("</p>");
        }

        println!("</section>");
    }
}

const TABLE_OF_CONTENTS: &str = "__TABLE_OF_CONTENTS__";
const PROJECTS: &str = "__PROJECTS__";

fn main() -> Result<()> {
    let template = std::fs::read_to_string("template.html")?;
    let projects = std::fs::read_to_string("projects.toml")?.parse::<toml::Table>()?;

    let table_of_contents_position = template
        .find(TABLE_OF_CONTENTS)
        .expect("__TABLE_OF_CONTENTS__ is missing from template file");
    let projects_position = template
        .find(PROJECTS)
        .expect("__PROJECTS__ is missing from template file");
    assert!(table_of_contents_position < projects_position);

    println!(
        "{header}",
        header = &template[0..table_of_contents_position]
    );
    table_of_contents(&projects);
    println!(
        "{mid}",
        mid = &template[(table_of_contents_position + TABLE_OF_CONTENTS.len())..projects_position]
    );
    sections(&projects);
    println!(
        "{footer}",
        footer = &template[(projects_position + PROJECTS.len())..]
    );

    Ok(())
}
