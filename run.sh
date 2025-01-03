#!/bin/sh

genjson(){
	jname=./sample.d/sample.jsonl

	test -f "${jname}" && rm "${jname}"

	jq \
		-c \
		-n '{
			name:"fuji",
			height:3.776,
			active:false,
			created:"2024-12-26T07:52:33.012345Z",
			updated:null
		}' |
	cat >> "${jname}"

	jq \
		-c \
		-n '{
			name:"takao",
			height:0.599,
			active:true,
			created:"2024-12-25T07:52:33.012345Z",
			updated:"2024-12-27T07:52:33.012345Z"
		}' |
	cat >> "${jname}"
}

genavro(){
	export ENV_SCHEMA_FILENAME=./sample.d/input.avsc

	cat sample.d/sample.jsonl |
		json2avrows |
		cat > ./sample.d/input.avro
}

runwasi0(){
	cat sample.d/input.avro |
		wazero run ./rs-avro-maps2keyset.wasm
}

runnative(){
	cat sample.d/input.avro |
		./rs-avro-maps2keyset
}

#genjson
#genavro

#runnative
runwasi0
