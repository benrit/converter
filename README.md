# config.toml

## Section \[machine\] 

### type
defines the type of import beeing use 
implemented values are "zeiss", "werth" e.g.
``` toml
type="zeiss"
```


### dmesn
defines the serial number of the machine e.g.
``` toml
dmesn="501717
```

### dmeswv
defines the software version
``` toml
dmeswv="6.8.2606"
```

### controllertyp="C32BIT"
defines the controllertyp of the machine
``` toml
controllertyp="C32BIT"
```

### dmeswi="Calypso"
defines the software
``` toml
dmeswi="Calypso"
```

### dmeid
defines the machine type
``` toml
dmeid="CONT_G2"
```


## Section \[configuation\]
path to where the cmm software exports the result files
``` toml
table_file_path=""
```


## Section \[\[export\]\]



## Example
``` toml
[machine]
type="zeiss"
dmesn="501717"
dmeswv="6.8.2606"
controllertyp="C32BIT"
dmeswi="Calypso"
dmeid="CONT_G2"


[[export]]
type="xml"
xml_result_path="O:\\Measurement\\results"

[[export]]
type="cmm"
cmm_result_path=".\\test_data"

[[export]]
type="mongodb"
uri="mongodb:\\urlToMongodb"
database="cmmData"
collection="results"

```