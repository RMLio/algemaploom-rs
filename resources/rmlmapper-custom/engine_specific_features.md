# Active Engine-Specific Test Cases from RML-Mapper

### CSVW (CSV on the Web) Tests  
- RMLTC0002a variants: comment-prefix, delimiter, encoding, tabs, unicode, trim
- RMLTC1002a variants: null, null-ignore, nulls
- RMLTC1025-missing-column-names-CSVW, RMLTC1035-bom-CSVW


### HTML Mapping Tests
- RMLTC0001a-HTML, RMLTC0015a-HTML

### ODS (OpenDocument Spreadsheet) Tests
- RMLTC0000-ODS through RMLTC0020b-ODS
- RMLTC1003-ODS through RMLTC1015-ODS

### Oracle Database Tests
- RMLTC0001a-OracleDB (basic test case)

### SPARQL Tests
- RMLTC0000-SPARQL, RMLTC0001a-SPARQL, RMLTC0001b-SPARQL, RMLTC0002a-SPARQL
- RMLTC0002g-SPARQL, RMLTC0003c-SPARQL through RMLTC0008c-SPARQL
- RMLTC0012a-SPARQL, RMLTC1029a-SPARQL

## Advanced Feature Tests

### RML I/O Tests
### LDES (Linked Data Event Stream) Tests
- rml-ldes/bluebike/base.rml.ttl
- rml-ldes/RMLLDES0001a through RMLLDES0003g
- web-of-things/ldes/generation tests: basic, repeat, partial

### Web of Things (WoT) Tests
- web-of-things/essence/, web-of-things/irail-stations/
- Security scheme tests: bearer-security-scheme-mocked, oauth2-security-scheme-mocked

### Target Mapping Tests
- SPARQL target, local file targets (VOID/DCAT), LDES targets
- Serialization tests: N-Quads, Turtle, N-Triples, JSON-LD, TriG

## Optimization and Utility Tests
### Mapping Optimizations
### HTTP/URL Tests
- Mapping file URL handling for various RDF formats
- Mocked URL response tests
