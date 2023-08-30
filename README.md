# TortankJS

Node addon to parse and manipulate n3/turtle data. 
Uses [Tortank](https://github.com/nbittich/tortank).

## Installation 

### Using the prebuilt node addon (GLIBC)

<b>This will only work if you are on linux and if you have GLIBC 2.31+ installed (check with `ldd --version`)</b>

#### Example using docker

- `docker run --rm -it node:16-bookworm bash`
- `mkdir example && cd example`
- `npm init --yes`
- `npm i rdf-tortank-linux`
- `node`
- `const tortank = require('rdf-tortank-linux')`

### Using the prebuilt node addon (MUSL)

<b>This will only work if you are on linux and if you have libc.musl-x86_64 installed (check with `ldd --version`)</b>

#### Example using docker

- `docker run --rm -it node:16-bookworm bash`
- `mkdir example && cd example`
- `npm init --yes`
- `npm i rdf-tortank-linux-musl`
- `node`
- `const tortank = require('rdf-tortank-linux-musl')`

### Using Rust

This is the preferred solution to target a different platform.

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install Node 
3. Inside your project `npm i --save-dev cargo-cp-artifact rdf-tortank`
4. `node`
5. `const tortank = require('rdf-tortank')`

#### Example using docker

- `docker run --rm -it rust bash`
- `apt update && apt upgrade -y`
- `curl -fsSL https://deb.nodesource.com/setup_16.x |  bash - && apt-get install -y nodejs`
- `mkdir example && cd example`
- `npm init --yes`
- `npm i --save-dev cargo-cp-artifact rdf-tortank`
- `node`
- `const tortank = require('rdf-tortank')`
## Documentation

### Statements

Filter a Model based on subject, predicate object. It uses same params as 
examples below, except there is no rhsPath / rhsData.

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
    outputType: "n3", // js|n3|undefined,  output type
    extraPrefixes: { // also optionals, if you need more prefixes to be defined
      ext: "http://example.org/show/",
    },
    wellKnownPrefix: undefined, // undefined | string, for anon nodes (https://www.w3.org/2011/rdf-wg/wiki/Skolemisation)
    subject: undefined, // uri|undefined, to filter subjects (must be an absolute uri)
    predicate: "<http://foaf.com/name>", // rdf iri|undefined, to filter predicates (muts be an absolute uri)
    object: '"Eve"', // rdf string | rdf iri | undefined, to filter objects

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


