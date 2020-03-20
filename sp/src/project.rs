pub mod project_boilerplate {
    use std::process::Command;

    extern crate serde_json;

    extern crate serde;
    use serde::ser::Serializer;

    pub fn config_text(project_name: &str) -> std::string::String {
        let get_python = Command::new("sh")
            .arg("which python")
            .output()
            .expect("Couldn't find python");

        let python_path = get_python.stdout;
        println!("Python found at: {}", String::from_utf8_lossy(&python_path));

        let config_string = json!({
            "project":project_name,
            "entrypoint":"index.py",
            "dependencies":[],
            "python_interpreter":String::from_utf8_lossy(&python_path)
        });

        return serde_json::to_string_pretty(&config_string).unwrap();
    }

    pub fn entrypoint_text() -> std::string::String {
        return String::from("import json
import csv
import pandas

class Index:
    def __init__(self):
        with open(\"./sp.json\", \"r\") as configs:
            self.config = json.load(configs)
        
        self.data = {}

    def load_data(self):
        for dataset in self.config[\"data\"]:
            data_object = self.data[dataset]
            if data_object[\"type\"] == \"csv\":
                self.data[dataset] = pandas.read_csv(data_object[\"file_path\"])
            elif data_object[\"type\"] == \"json\":
                self.data[dataset] = json.load(data_object[\"file_path\"])
            else:
                continue

    def load_models(self):
        for model in self.config[\"workflow\"][\"models\"]:
            try:
                model_path = \"models.\" + model[\"name\"]
                __import__(model_path)
            except expression as identifier:
                pass

    def run_workflow(self):
        pass          

project = Index()");
    }
}