use std::{
	collections::{hash_map, HashMap},
	env::{self},
	fs::File,
	io::Read,
	path::Path,
};

use enwn2odict::wn::{LexicalResource, Pronunciation, Synset};
use odict::{
	Definition, DefinitionType, Dictionary, DictionaryWriter, Entry, Etymology, Example, Note,
	PartOfSpeech, Sense, ID,
};

fn main() {
	let args: Vec<String> = env::args().collect();
	let wn_path = Path::new(&args[1]);
	let od_path = Path::new(&args[2]);

	let mut wn_file = File::open(wn_path).unwrap();
	let mut wn_xml = String::new();
	wn_file.read_to_string(&mut wn_xml).unwrap();

	println!(
		"Deserializing English WordNet XML ({} bytes)...",
		wn_xml.len()
	);
	let wn = quick_xml::de::from_str::<LexicalResource>(&wn_xml).unwrap();
	let wn_lexicon = wn.lexicon;
	let wn_synset_cnt = wn_lexicon.synset.len();
	let wn_entry_cnt = wn_lexicon.lexical_entry.len();

	println!("Collecting synonyms from English WordNet synsets ({wn_entry_cnt})...");
	let mut wn_id2synonyms = HashMap::<String, Vec<String>>::new();
	for wn_entry in &wn_lexicon.lexical_entry {
		let term = &wn_entry.lemma.written_form;
		for wn_sense in &wn_entry.sense {
			let id = &wn_sense.synset;
			if let hash_map::Entry::Vacant(e) = wn_id2synonyms.entry(id.clone()) {
				e.insert(vec![term.clone()]);
			} else {
				wn_id2synonyms.get_mut(id).unwrap().push(term.clone());
			}
		}
	}

	println!("Building ODict definitions from English WordNet synsets ({wn_synset_cnt})...");
	let mut id2definition = HashMap::new();
	for synset in &wn_lexicon.synset {
		let id = &synset.id;
		assert!(!id2definition.contains_key(id));
		id2definition.insert(id.clone(), build_definition(synset, &wn_id2synonyms[id]));
	}

	println!("Building ODict entries from English WordNet lexical entries ({wn_entry_cnt})...");
	let mut entries: HashMap<String, Entry> = HashMap::new();
	let mut form2root_forms: HashMap<String, Vec<String>> = HashMap::new();
	for wn_entry in &wn_lexicon.lexical_entry {
		let term = &wn_entry.lemma.written_form;
		let pos = parse_pos(&wn_entry.lemma.part_of_speech);
		assert!(pos != PartOfSpeech::un);
		let pronunciation = build_pronunciation(&wn_entry.lemma.pronunciation);

		let entry = if let Some(entry) = entries.get_mut(term) {
			entry
		} else {
			entries.insert(
				term.clone(),
				Entry {
					term: term.clone(),
					see_also: None,
					etymologies: Vec::new(),
				},
			);

			entries.get_mut(term).unwrap()
		};

		// TODO: replace this with a function
		let mut senses = HashMap::new();
		let mut definitions = Vec::new();
		// ODict divides senses by part of speech, so we must combine WordNet senses into one.
		for wn_sense in &wn_entry.sense {
			let synset_id = &wn_sense.synset;
			let definition = id2definition[synset_id].clone();
			definitions.push(DefinitionType::Definition(definition));
		}
		senses.insert(pos.clone(), Sense { pos, definitions });

		let mut forms = term.clone();
		if let Some(wn_forms) = &wn_entry.form {
			for form in wn_forms.iter().map(|f| &f.written_form) {
				forms.push_str(", ");
				forms.push_str(form);

				if !form2root_forms.contains_key(form) {
					form2root_forms.insert(form.clone(), Vec::new());
				}
				form2root_forms.get_mut(form).unwrap().push(term.clone());
			}
		}

		// WordNet doesn't have information of etymology, so we consider each
		// wn_entry belongs to one etymology.
		let ety = Etymology {
			id: Some(wn_entry.id.clone()),
			pronunciation: pronunciation.clone(),
			// We don't have to good place to put the variant forms, here OK for now.
			description: Some(forms),
			senses,
		};

		entry.etymologies.push(ety);
	}

	println!(
		"Adding see also information from variant to root form ({})...",
		form2root_forms.len()
	);
	for (form, root_forms) in &form2root_forms {
		add_see_also(&mut entries, form, root_forms);
	}

	let dict = Dictionary {
		id: ID::new(), // TBD
		name: Some(wn_lexicon.label.clone()),
		entries,
	};

	let writer = DictionaryWriter::new();
	println!(
		"Writing the dictionary of {} entries...",
		dict.entries.len()
	);
	writer.write_to_path(&dict, od_path).unwrap();

	println!("Done.");
}

/// Add see also information from variant to root forms
fn add_see_also(entries: &mut HashMap<String, Entry>, form: &String, root_forms: &[String]) {
	let see_also_info = format!("See also: {}", root_forms.join(", "));
	// ODict doesn't have a dedicated way to add see also information, so we use
	// an empty etymology with see also description for workaround
	let ety = Etymology {
		id: None, // no synset, therefore no ID
		pronunciation: None,
		description: Some(see_also_info),
		senses: HashMap::new(),
	};

	if !entries.contains_key(form) {
		entries.insert(
			form.clone(),
			Entry {
				term: form.clone(),
				see_also: None,
				etymologies: Vec::new(),
			},
		);
	}
	let entry = entries.get_mut(form).unwrap();
	entry.etymologies.push(ety);

	// ODict doesn't support multiple see also alias, so we only add alias in one to one case
	if root_forms.len() == 1 {
		entry.see_also = Some(root_forms[0].clone());
	}
}

// TODO: ODict CLI seems not using this
fn build_pronunciation(pronunciation: &Option<Vec<Pronunciation>>) -> Option<String> {
	if let Some(pronunciations) = pronunciation {
		let pronunciation = pronunciations
			.iter()
			.map(|p| {
				assert!(p.text.is_some());
				let text = p.text.as_ref().unwrap();
				if let Some(variety) = &p.variety {
					format!("{variety}: {text}")
				} else {
					text.clone()
				}
			})
			.collect::<Vec<String>>()
			.join("; ");

		Some(pronunciation)
	} else {
		None
	}
}

fn build_definition(synset: &Synset, synonyms: &[String]) -> Definition {
	let id = &synset.id;
	// WordNet may provides multiple definitions, but they are of the same meaning.
	// And we should combine them to use the same examples.
	let value = synset.definition.join("; ");

	let mut examples = Vec::new();
	if let Some(wn_examples) = &synset.example {
		for wn_example in wn_examples {
			if let Some(text) = &wn_example.text {
				examples.push(Example {
					value: text.clone(),
				});
			}
		}
	}

	let mut notes = Vec::new();
	if synonyms.len() > 1 {
		notes.push(build_synonym_note(synonyms));
	}

	Definition {
		id: Some(id.clone()),
		value,
		examples,
		notes,
	}
}

fn build_synonym_note(synonyms: &[String]) -> Note {
	Note {
		id: None,
		value: format!("Synonyms: {}", synonyms.join(", ")),
		examples: Vec::new(),
	}
}

/// Parse WordNet position of speech into ODict type
///
/// # WordNet Part of Speech
///
/// n: Noun => n: noun
/// v: Verb => v: verb
/// a: Adjective => adj: adjective
/// r: Adverb => adv: adverb
/// s: Adjective Satellite => adj: adjective
/// z: Multiword expression (inc. phrase, idiom) => phr: phrase
/// c: Conjunction: conj: conjunction
/// p: Adposition (Preposition, postposition, etc.) => prep + postp
/// x: Other (inc. particle, classifier, bound morpheme, determiner) => part + det + ...
/// u: Unknown
fn parse_pos(pos: &str) -> PartOfSpeech {
	match pos {
		"n" => PartOfSpeech::n,
		"v" => PartOfSpeech::v,
		"a" | "s" => PartOfSpeech::adj,
		"r" => PartOfSpeech::adv,
		"z" => PartOfSpeech::phr,
		"c" => PartOfSpeech::conj,
		"p" | "x" | "u" => PartOfSpeech::un,
		_ => panic!("expected a valid WordNet part of speech"),
	}
}
