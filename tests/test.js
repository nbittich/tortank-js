const { statements, difference, intersection, merge } = require('..');
const assert = require('assert');
const data = `
          @prefix foaf: <http://foaf.com/>.
          @prefix test: <http://bittich.be/>.

            [ foaf:name "Alice" ] foaf:knows [
              foaf:name "Bob" ;
              foaf:lastName "George", "Joshua" ;
              foaf:knows test:Eve;
              foaf:mbox <bob@example.com>
            ] .
            test:Eve   foaf:name "Eve"  .

`;

describe('Statements', () => {
  it('should be equal', () => {
    const params = {
      lhsData: data,
      outputType: "js",
      subject: undefined,
      predicate: "<http://foaf.com/name>",
      object: '"Eve"'
    };
    const actual = statements(params);
    assert.deepEqual([
      {
        "object": {
          "datatype": "http://www.w3.org/2001/XMLSchema#string",
          "type": "literal",
          "value": "Eve",
        },
        "predicate": {
          "type": "uri",
          "value": "http://foaf.com/name"
        },
        "subject": {
          "type": "uri",
          "value": "http://bittich.be/Eve"
        },
      }
    ], actual);
  });
  it('should still be equal after being transformed to json', () => {
    const params = {
      lhsData: data,
      outputType: "js",
      subject: undefined,
      predicate: "<http://foaf.com/name>",
      object: '"Eve"'
    };
    const actual = statements(params);
    params.lhsData = actual;
    const expected = statements(params);
    assert.deepEqual(expected, actual);
  });

  it('should still equal while using prefixes', () => {
    const params = {
      lhsData: data,
      outputType: "js",
      subject: "test:Eve",
      predicate: "foaf:name",
      object: undefined
    };
    const actual = statements(params);
    assert.deepEqual([{
      "object": {
        "datatype": "http://www.w3.org/2001/XMLSchema#string",
        "type": "literal",
        "value": "Eve",
      },
      "predicate": {
        "type": "uri",
        "value": "http://foaf.com/name"
      },
      "subject": {
        "type": "uri",
        "value": "http://bittich.be/Eve"
      },
    }], actual);
  });
});

describe('Difference', () => {
  it("should find differences while using file", () => {
    const paramsForDiff = {
      lhsPath: "../example/modelA.ttl",
      rhsPath: "../example/modelB.ttl",
      outputType: "n3",
      outputFilePath: "/tmp/diff.ttl",
      bufSize: 10
    }
    difference(paramsForDiff);

    const paramsForStmts = {
      lhsPath: "/tmp/diff.ttl",
      outputType: "js"
    };

    const actual = statements(paramsForStmts);
    assert.deepEqual([
      {
        subject: { value: 'mailto:person@example.net', type: 'uri' },
        predicate: { value: 'http://xmlns.com/foaf/0.1/name', type: 'uri' },
        object: {
          value: 'Anne Example-Person',
          type: 'literal',
          datatype: 'http://www.w3.org/2001/XMLSchema#string'
        }
      }
    ], actual);

    const paramsForDiffBetweenPathAndJs = {
      lhsPath: "/tmp/diff.ttl",
      rhsData: "actual",
      outputType: "js",
    };
    assert.deepEqual(actual, difference(paramsForDiffBetweenPathAndJs));
  });

  it("should find differences mixing up stuff", () => {
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
      lhsData, rhsPath: "../example/modelC.ttl",
    };
    const actual = difference(paramsByDataAndPath);
    assert.equal(8, actual.length);

    const inverseParams = {
      rhsData: lhsData,
      lhsPath: "../example/modelC.ttl"
    };
    const otherActual = difference(inverseParams);
    assert.equal(2, otherActual.length);
  });


});

describe('Intersection', () => {
  it("should find intersection while using files", () => {
    const params = {
      lhsPath: "../example/modelA.ttl",
      rhsPath: "../example/modelB.ttl",
      outputType: "js",
    };
    const res = intersection(params);

    assert.deepEqual([
      {
        subject: { value: 'mailto:person@example.net', type: 'uri' },
        predicate: {
          value: 'http://www.w3.org/1999/02/22-rdf-syntax-ns#type',
          type: 'uri'
        },
        object: { value: 'http://xmlns.com/foaf/0.1/Person', type: 'uri' }
      },
      {
        subject: { value: 'mailto:person@example.net', type: 'uri' },
        predicate: { value: 'http://xmlns.com/foaf/0.1/interest', type: 'uri' },
        object: { value: 'http://www.foaf-project.org/', type: 'uri' }
      },
      {
        subject: { value: 'mailto:person@example.net', type: 'uri' },
        predicate: { value: 'http://xmlns.com/foaf/0.1/interest', type: 'uri' },
        object: {
          value: 'http://www.ilrt.bris.ac.uk/discovery/2004/01/turtle/',
          type: 'uri'
        }
      }
    ], res);
  });
});

describe("Merge", () => {
  it("should merge model A and model C", () => {
    const params = {
      lhsPath: "../example/modelA.ttl",
      rhsPath: "../example/modelC.ttl",
      outputType: "n3",
      outputFilePath: "/tmp/merge.ttl"
    };
    merge(params);

    const params2 = {
      lhsPath: "../example/modelA.ttl",
      rhsPath: "../example/modelC.ttl",
      outputType: "js",
      outputFilePath: "/tmp/merge.json"
    };
    merge(params2);

    const params3 = {
      lhsPath: "/tmp/merge.ttl",
      rhsPath: "/tmp/merge.json",
    };

    let res = intersection(params3);
    assert.deepEqual([
      {
        subject: { value: 'mailto:person@example.net', type: 'uri' },
        predicate: {
          value: 'http://www.w3.org/1999/02/22-rdf-syntax-ns#type',
          type: 'uri'
        },
        object: { value: 'http://xmlns.com/foaf/0.1/Person', type: 'uri' }
      },
      {
        subject: { value: 'mailto:person@example.net', type: 'uri' },
        predicate: { value: 'http://xmlns.com/foaf/0.1/name', type: 'uri' },
        object: {
          value: 'Anne Example-Person',
          type: 'literal',
          datatype: 'http://www.w3.org/2001/XMLSchema#string'
        }
      },
      {
        subject: { value: 'mailto:person@example.net', type: 'uri' },
        predicate: { value: 'http://xmlns.com/foaf/0.1/interest', type: 'uri' },
        object: { value: 'http://www.foaf-project.org/', type: 'uri' }
      },
      {
        subject: { value: 'mailto:person@example.net', type: 'uri' },
        predicate: { value: 'http://xmlns.com/foaf/0.1/interest', type: 'uri' },
        object: {
          value: 'http://www.ilrt.bris.ac.uk/discovery/2004/01/turtle/',
          type: 'uri'
        }
      },
      {
        subject: { value: 'http://example.org/show/218', type: 'uri' },
        predicate: { value: 'http://example.org/show/localName', type: 'uri' },
        object: { value: 'That Seventies Show', type: 'literal', lang: 'en' }
      },
      {
        subject: { value: 'http://bittich.be/some/url/1233', type: 'uri' },
        predicate: { value: 'http://example.org/firstName', type: 'uri' },
        object: { value: 'http://n.com/nordine', type: 'uri' }
      }
    ], res);




  });
});