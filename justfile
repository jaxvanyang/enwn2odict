name := "enwn2odict"
wn_ver := "2024"
minor_ver := "1"
wn_xml := "english-wordnet-" + wn_ver + ".xml"
wn_tarball := wn_xml + ".gz"
wn_source := "https://en-word.net/static/" + wn_tarball
odict := name + "-" + wn_ver + "." + minor_ver + ".odict"
odxml := name + "-" + wn_ver + "." + minor_ver + ".xml"

default: convert dump

prepare:
	curl -OL {{wn_source}} && gzip -d {{wn_tarball}}

gen-code:
	xml_schema_generator -d "Serialize, Deserialize, Debug, Clone" {{wn_xml}} src/wn.rs
	rustfmt src/wn.rs

convert:
	cargo run --release -- {{wn_xml}} {{odict}}

dump:
	odict dump -o {{odxml}} {{odict}}
