# TortankJS

Node addon to parse and manipulate n3/turtle data. 
Use [Tortank](https://github.com/nbittich/tortank).

## Installation 

TODO

## Documentation

### Difference

Creates a new, indepependent, model containing all the statements in the left model that are not in the right model.

```js

// diff between model a and model b, store result in a  file
const paramsByPath = {
    lhsPath: "./example/modelA.ttl", // string|undefined, to load the left model by file, if not provided, use lhsData
    rhsPath: "./example/modelB.ttl", // string|undefined, to load the right model by file, if not provided, use rhsData
    outputType: "n3", // either n3|json|undefined
    outputFilePath: "/tmp/diff.ttl", // string|undefined, if you want to save output directly into a file
    bufSize: 10 // number|undefined, optional, if outputFilePath is set, buffering 
}

try {
    tortank.difference(paramsByPath); // check content in /tmp/diff.ttl
}catch(e) {
    console.log("error! ", e);
}


// diff between model a and model b, store result in memory as javascipt object

const lhsData = `
      @prefix foaf: <http://foaf.com/>.
        [ foaf:name "Alice" ] foaf:knows [
          foaf:name "Bob" ;
          foaf:lastName "George", "Joshua" ;
          foaf:knows [
          foaf:name "Eve" ] ;
    foaf:mbox <bob@example.com>] .
`;
const paramsByDataAndPath = {
    lhsData, // string|undefined, to load the left model by file, if not provided, use lhsData
    rhsPath: "./example/modelC.ttl", // string|undefined, to load the right model by file, if not provided, use rhsData
}

try {
    let data = tortank.difference(paramsByDataAndPath); 
    console.log(data);
}catch(e) {
    console.log("error! ", e);
}


```



### Intersection

Creates a new, indepependent, model containing all the statements in the left model that are also in the right model.

The parameters are exactly similar to difference (see example above).

```js
try {
    tortank.intersection(paramsByDataAndPath); 
}catch(e) {
    console.log("error! ", e);
}
```

### Merge

Merge two models togeter. 
The parameters are exactly similar to difference and intersection (see example above).

```js
try {
    tortank.merge(paramsByDataAndPath); 
}catch(e) {
    console.log("error! ", e);
}
```

### Statements

Filter a Model based on subject, predicate object. It uses same params as previous
examples, except there is no rhsPath / rhsData.

```js
const data = `
      @prefix foaf: <http://foaf.com/>.
        [ foaf:name "Alice" ] foaf:knows [
          foaf:name "Bob" ;
          foaf:lastName "George", "Joshua" ;
          foaf:knows [
          foaf:name "Eve" ] ;
    foaf:mbox <bob@example.com>] .
`;

let params = {
    lhsData: data, // string|undefined, if not provided use lhsPath
    outputType: "n3", // also optional
    subject: undefined, // uri|undefined, to filter subjects (must be an absolute uri)
    predicate: "<http://foaf.com/name>", // rdf iri|undefined, to filter predicates (muts be an absolute uri)
    object: '"Eve"' // rdf string | rdf iri | undefined, to filter objects
};

tortank.statements(params);

```

You can also use prefixes, assuming they are known by the model. In the previous example, you could also do this:

```js
let paramsWithPrefix = {
    lhsData: data, 
    outputType: "js", 
    subject: undefined, 
    predicate: "foaf:lastName", // use prefix foaf
    object: undefined // rdf string | rdf iri | undefined, to filter objects
};
tortank.statements(params);


```

## Examples

const tortank = require('.');
const payloadCompare = {
    lhsPath: "./example/modelA.ttl",
    rhsPath: "./example/modelB.ttl"
}

let diff = tortank.difference(payloadCompare);
let intersection =tortank.intersection(payloadCompare);


let payloadFilter = {
  ttlPath: "./example/input.ttl",
  subject: "http://publications.europa.eu/resource/authority/country/ZWE",
  predicate: undefined,
  object: undefined
}

let triples = tortank.statements(payloadFilter)

