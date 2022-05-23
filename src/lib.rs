pub mod bus_lines {
    use reqwest::{self, Error as RequestError};
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs::read_to_string;

    #[derive(Deserialize, Debug, Default)]
    pub struct Street {
        pub id: String,
        pub desc: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Line {
        pub line_id: String,
        pub name: String,
    }

    #[derive(Default)]
    pub struct Stop {}

    #[derive(Deserialize)]
    pub struct LineCollection {
        lines: Vec<Line>,
    }

    pub trait ListDisplay {
        fn get_text(&self) -> &str;
    }

    impl ListDisplay for Street {
        fn get_text(&self) -> &str {
            &self.desc
        }
    }

    impl ListDisplay for Line {
        fn get_text(&self) -> &str {
            &self.name
        }
    }

    enum Action {
        GetStreet,
        GetIntersection,
    }

    pub enum RequestAction<'a> {
        Street(&'a Line),
        Intersection(&'a Line, &'a Street),
    }

    pub fn get_lines<'a>() -> Result<Vec<Line>, Box<dyn Error>> {
        let file_contents = read_to_string("./src/lineas.json")?;
        let lines: LineCollection = serde_json::from_str(&file_contents)?;

        Ok(lines.lines)
    }

    pub fn get_streets<'a>(
        request_action: &'a RequestAction<'a>,
        mut params: HashMap<&str, &'a str>,
    ) -> Result<Vec<Street>, RequestError> {
        let client = reqwest::blocking::Client::new();

        match request_action {
            RequestAction::Street(line) => {
                params.insert("accion", "getCalle");
                params.insert("idLinea", line.line_id.as_str());
            }
            RequestAction::Intersection(line, street) => {
                params.insert("accion", "getInterseccion");
                params.insert("idLinea", line.line_id.as_str());
                params.insert("idCalle", street.id.as_str());
            }
        };

        let res = client
            .post("http://www.etr.gov.ar/ajax/cuandollega/getInfoParadas.php")
            .form(&params)
            .send()?;

        let stops: Vec<Street> = res.json()?;

        Ok(stops)
    }
}

pub mod app_state {}
