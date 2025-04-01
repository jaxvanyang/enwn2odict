use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LexicalResource {
	#[serde(rename = "@xmlns:dc")]
	pub xmlns_dc: String,
	#[serde(rename = "$text")]
	pub text: Option<String>,
	#[serde(rename = "Lexicon")]
	pub lexicon: Lexicon,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lexicon {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "@label")]
	pub label: String,
	#[serde(rename = "@language")]
	pub language: String,
	#[serde(rename = "@email")]
	pub email: String,
	#[serde(rename = "@license")]
	pub license: String,
	#[serde(rename = "@version")]
	pub version: String,
	#[serde(rename = "@url")]
	pub url: String,
	#[serde(rename = "$text")]
	pub text: Option<String>,
	#[serde(rename = "LexicalEntry")]
	pub lexical_entry: Vec<LexicalEntry>,
	#[serde(rename = "Synset")]
	pub synset: Vec<Synset>,
	#[serde(rename = "SyntacticBehaviour")]
	pub syntactic_behaviour: Vec<SyntacticBehaviour>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LexicalEntry {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text")]
	pub text: Option<String>,
	#[serde(rename = "Lemma")]
	pub lemma: Lemma,
	#[serde(rename = "Sense")]
	pub sense: Vec<Sense>,
	#[serde(rename = "Form")]
	pub form: Option<Vec<Form>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lemma {
	#[serde(rename = "@writtenForm")]
	pub written_form: String,
	#[serde(rename = "@partOfSpeech")]
	pub part_of_speech: String,
	#[serde(rename = "$text")]
	pub text: Option<String>,
	#[serde(rename = "Pronunciation")]
	pub pronunciation: Option<Vec<Pronunciation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pronunciation {
	#[serde(rename = "@variety")]
	pub variety: Option<String>,
	#[serde(rename = "$text")]
	pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sense {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "@synset")]
	pub synset: String,
	#[serde(rename = "@subcat")]
	pub subcat: Option<String>,
	#[serde(rename = "@adjposition")]
	pub adjposition: Option<String>,
	#[serde(rename = "$text")]
	pub text: Option<String>,
	#[serde(rename = "SenseRelation")]
	pub sense_relation: Option<Vec<SenseRelation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SenseRelation {
	#[serde(rename = "@relType")]
	pub rel_type: String,
	#[serde(rename = "@target")]
	pub target: String,
	#[serde(rename = "@type")]
	pub dc_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Form {
	#[serde(rename = "@writtenForm")]
	pub written_form: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Synset {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "@ili")]
	pub ili: String,
	#[serde(rename = "@members")]
	pub members: String,
	#[serde(rename = "@partOfSpeech")]
	pub part_of_speech: String,
	#[serde(rename = "@lexfile")]
	pub lexfile: String,
	#[serde(rename = "@source")]
	pub dc_source: Option<String>,
	#[serde(rename = "$text")]
	pub text: Option<String>,
	#[serde(rename = "Definition")]
	pub definition: Vec<String>,
	#[serde(rename = "SynsetRelation")]
	pub synset_relation: Option<Vec<SynsetRelation>>,
	#[serde(rename = "Example")]
	pub example: Option<Vec<Example>>,
	#[serde(rename = "ILIDefinition")]
	pub ilidefinition: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SynsetRelation {
	#[serde(rename = "@relType")]
	pub rel_type: String,
	#[serde(rename = "@target")]
	pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Example {
	#[serde(rename = "@source")]
	pub dc_source: Option<String>,
	#[serde(rename = "$text")]
	pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyntacticBehaviour {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "@subcategorizationFrame")]
	pub subcategorization_frame: String,
}
