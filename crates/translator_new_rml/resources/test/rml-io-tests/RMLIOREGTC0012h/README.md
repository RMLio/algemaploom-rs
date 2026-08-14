## RMLIOREGTC0012h

**Title**: CSVW BOM

**Description**: BOM value in CSV

**Error expected?** No

**Input**
 [http://w3id.org/rml/resources/rml-io/RMLIOREGTC0012h/Friends.json](http://w3id.org/rml/resources/rml-io/RMLIOREGTC0012h/Friends.json)

**Mapping**
```
@prefix rml: <http://w3id.org/rml/> .
@prefix csvw: <http://www.w3.org/ns/csvw#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#>.
@prefix schema: <http://schema.org/>.
@prefix wgs84_pos: <http://www.w3.org/2003/01/geo/wgs84_pos#lat>.
@prefix gn: <http://www.geonames.org/ontology#>.
@prefix carml: <http://carml.taxonic.com/carml/> .
@prefix fnml: <http://semweb.mmlab.be/ns/fnml#> .
@prefix grel: <http://users.ugent.be/~bjdmeest/function/grel.ttl#> .
@prefix fno: <https://w3id.org/function/ontology#> .
@base <http://example.com/ns#>.

<#LogicalSourceGrant> a rml:BaseSource ;
  rml:source <#CSVW_sourceGrant> ;
  rml:referenceFormulation rml:CSV .

<#CSVW_sourceGrant> a csvw:Table;
   csvw:url "data.csv" ;
   csvw:dialect [ a csvw:Dialect;
       csvw:delimiter ";"
   ] .

<#ProjectMapping> a rml:TriplesMap;
  rml:logicalSource <#LogicalSourceGrant> ;

  rml:subjectMap [
    rml:template "http://snf.ch/project/{GrantNumber}";
    rml:class schema:ResearchProject
  ] ;

  rml:predicateObjectMap [
    rml:predicate schema:description ;
    rml:objectMap [
      rml:reference "Abstract" # first column's name
    ]
  ] .

```

**Output**
```
<http://snf.ch/project/100348> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/100348> <http://schema.org/description> "".
<http://snf.ch/project/147091> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/147091> <http://schema.org/description> "Non-recombining sex chromosomes are expected to accumulate deleterious mutations under the combined effects of Muller’s ratchet and Hill-Robertson effects, and for this reason to progressively degenerate. Such decay has been largely documented in birds and mammals, most of which have strongly heteromorphic sex chromosomes with degenerated W and Y, respectively. In sharp contrast, however, cold-blooded vertebrates rarely show such a differentiation, even when sex determination is purely genetic. The present project aims at investigating the evolutionary dynamics of non-degenerating sex chromosomes, using amphibians as model organisms.".
<http://snf.ch/project/28381> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/28381> <http://schema.org/description> "".
<http://snf.ch/project/48354> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/48354> <http://schema.org/description> "".
<http://snf.ch/project/38943> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/38943> <http://schema.org/description> "".
<http://snf.ch/project/55475> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/55475> <http://schema.org/description> "".
<http://snf.ch/project/20770> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/20770> <http://schema.org/description> "".
<http://snf.ch/project/38762> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/38762> <http://schema.org/description> "".
<http://snf.ch/project/108100> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/ResearchProject>.
<http://snf.ch/project/108100> <http://schema.org/description> "".


```

