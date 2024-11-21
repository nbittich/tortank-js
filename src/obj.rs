use std::{collections::BTreeMap, path::PathBuf};

use neon::prelude::*;
use tortank::turtle::turtle_doc::{
    RdfJsonNode, RdfJsonNodeResult, RdfJsonTriple, Statement, TurtleDoc, TurtleDocError,
};

const PARAMS_LHS_PATH: &str = "lhsPath";
const PARAMS_RHS_PATH: &str = "rhsPath";
const PARAMS_LHS_DATA: &str = "lhsData";
const PARAMS_RHS_DATA: &str = "rhsData";
const PARAMS_SUBJECT_NODE: &str = "subject";
const PARAMS_PREDICATE_NODE: &str = "predicate";
const PARAMS_OBJECT_NODE: &str = "object";
const PARAMS_OUTPUT_TYPE: &str = "outputType";
const PARAMS_OUTPUT_FILE_PATH: &str = "outputFilePath";
const PARAMS_BUF_SIZE: &str = "bufSize";
const PARAMS_EXTRA_PREFIXES: &str = "extraPrefixes";
const PARAMS_WELL_KNOWN_PREFIX: &str = "wellKnownPrefix";
const PARAMS_MAPPER_FUNCTION: &str = "mapperFunction";

pub enum DocType<'a> {
    TurtleDoc(TurtleDoc<'a>),
    RdfJsonTriple((Vec<RdfJsonTriple>, BTreeMap<String, String>)),
}

pub fn merge(mut cx: FunctionContext) -> JsResult<JsValue> {
    let params = cx.argument::<JsObject>(0)?;
    let mut buf_lhs = String::new();
    let mut buf_rhs = String::new();
    let ttl_doc_lhs = make_doc(
        &params,
        &mut cx,
        &mut buf_lhs,
        PARAMS_LHS_PATH,
        PARAMS_LHS_DATA,
    );
    let ttl_doc_rhs = make_doc(
        &params,
        &mut cx,
        &mut buf_rhs,
        PARAMS_RHS_PATH,
        PARAMS_RHS_DATA,
    );

    match (ttl_doc_lhs, ttl_doc_rhs) {
        (Ok(DocType::TurtleDoc(lhs)), Ok(DocType::TurtleDoc(rhs))) => {
            make_response(&params, &mut cx, lhs + rhs)
        }
        (Ok(DocType::TurtleDoc(lhs)), Ok(DocType::RdfJsonTriple((rhs, prefixes)))) => {
            match rdf_json_triple_to_doc(&rhs[..], prefixes) {
                Ok(doc) => make_response(&params, &mut cx, lhs + doc),
                Err(e) => cx.throw_error(e.message),
            }
        }
        (Ok(DocType::RdfJsonTriple((lhs, prefixes))), Ok(DocType::TurtleDoc(rhs))) => {
            match rdf_json_triple_to_doc(&lhs[..], prefixes) {
                Ok(doc) => make_response(&params, &mut cx, doc + rhs),
                Err(e) => cx.throw_error(e.message),
            }
        }
        (
            Ok(DocType::RdfJsonTriple((lhs, lhs_prefixes))),
            Ok(DocType::RdfJsonTriple((rhs, rhs_prefixes))),
        ) => {
            match (
                rdf_json_triple_to_doc(&lhs[..], lhs_prefixes),
                rdf_json_triple_to_doc(&rhs[..], rhs_prefixes),
            ) {
                (Ok(lhs), Ok(rhs)) => make_response(&params, &mut cx, lhs + rhs),
                (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.message),
                (Err(e1), Err(e2)) => {
                    cx.throw_error(format!("error:\n-{}\n-{}", e1.message, e2.message))
                }
            }
        }
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.message),
        (Err(e1), Err(e2)) => cx.throw_error(format!("error:\n-{}\n-{}", e1.message, e2.message)),
    }
}
pub fn difference(mut cx: FunctionContext) -> JsResult<JsValue> {
    let params = cx.argument::<JsObject>(0)?;

    let mut buf_lhs = String::new();
    let mut buf_rhs = String::new();
    let ttl_doc_lhs = make_doc(
        &params,
        &mut cx,
        &mut buf_lhs,
        PARAMS_LHS_PATH,
        PARAMS_LHS_DATA,
    );
    let ttl_doc_rhs = make_doc(
        &params,
        &mut cx,
        &mut buf_rhs,
        PARAMS_RHS_PATH,
        PARAMS_RHS_DATA,
    );
    fn diff_fn<'a, 'b, C: Context<'a>>(
        params: &'b Handle<'b, JsObject>,
        cx: &mut C,
        lhs: TurtleDoc<'b>,
        rhs: TurtleDoc<'b>,
    ) -> JsResult<'a, JsValue> {
        let diff = lhs.difference(&rhs);
        match diff {
            Ok(model) => make_response(params, cx, model),

            Err(e) => cx.throw_error(e.to_string()),
        }
    }
    match (ttl_doc_lhs, ttl_doc_rhs) {
        (Ok(DocType::TurtleDoc(lhs)), Ok(DocType::TurtleDoc(rhs))) => {
            diff_fn(&params, &mut cx, lhs, rhs)
        }
        (Ok(DocType::TurtleDoc(lhs)), Ok(DocType::RdfJsonTriple((rhs, prefixes)))) => {
            match rdf_json_triple_to_doc(&rhs[..], prefixes) {
                Ok(doc) => diff_fn(&params, &mut cx, lhs, doc),
                Err(e) => cx.throw_error(e.message),
            }
        }
        (Ok(DocType::RdfJsonTriple((lhs, lhs_prefixes))), Ok(DocType::TurtleDoc(rhs))) => {
            match rdf_json_triple_to_doc(&lhs[..], lhs_prefixes) {
                Ok(doc) => diff_fn(&params, &mut cx, doc, rhs),
                Err(e) => cx.throw_error(e.message),
            }
        }
        (
            Ok(DocType::RdfJsonTriple((lhs, lhs_prefixes))),
            Ok(DocType::RdfJsonTriple((rhs, rhs_prefixes))),
        ) => {
            match (
                rdf_json_triple_to_doc(&lhs[..], lhs_prefixes),
                rdf_json_triple_to_doc(&rhs[..], rhs_prefixes),
            ) {
                (Ok(lhs), Ok(rhs)) => diff_fn(&params, &mut cx, lhs, rhs),
                (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.message),
                (Err(e1), Err(e2)) => {
                    cx.throw_error(format!("error:\n-{}\n-{}", e1.message, e2.message))
                }
            }
        }
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.message),
        (Err(e1), Err(e2)) => cx.throw_error(format!("error:\n-{}\n-{}", e1.message, e2.message)),
    }
}

fn filter_statements<'a, 'b, C: Context<'a>>(
    params: &'b Handle<'b, JsObject>,
    cx: &mut C,
    ttl_doc: &'b TurtleDoc<'b>,
) -> Result<TurtleDoc<'b>, TurtleDocError> {
    let subject: Option<String> = params
        .get_opt::<JsString, _, _>(cx, PARAMS_SUBJECT_NODE)
        .map(|s| {
            if let Some(s) = s {
                Some(s.value(cx))
            } else {
                None
            }
        })
        .map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;

    let predicate: Option<String> = params
        .get_opt::<JsString, _, _>(cx, PARAMS_PREDICATE_NODE)
        .map(|s| {
            if let Some(s) = s {
                Some(s.value(cx))
            } else {
                None
            }
        })
        .map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;

    let object: Option<String> = params
        .get_opt::<JsString, _, _>(cx, PARAMS_OBJECT_NODE)
        .map(|s| {
            if let Some(s) = s {
                Some(s.value(cx))
            } else {
                None
            }
        })
        .map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;

    let stmts: Vec<&Statement> = ttl_doc.parse_and_list_statements(subject, predicate, object)?;

    TurtleDoc::try_from(stmts)
}

pub fn statements(mut cx: FunctionContext) -> JsResult<JsValue> {
    let params = cx.argument::<JsObject>(0)?;

    let mut buf = String::new();

    let ttl_doc = make_doc(&params, &mut cx, &mut buf, PARAMS_LHS_PATH, PARAMS_LHS_DATA);

    match ttl_doc {
        Ok(ttl_doc) => {
            let doc = match ttl_doc {
                DocType::TurtleDoc(doc) => Ok(doc),
                DocType::RdfJsonTriple((ref rjs, prefixes)) => {
                    match rdf_json_triple_to_doc(&rjs[..], prefixes) {
                        Ok(doc) => Ok(doc),
                        Err(e) => cx.throw_error(e.message),
                    }
                }
            }?;
            match filter_statements(&params, &mut cx, &doc) {
                Ok(doc) => make_response(&params, &mut cx, doc),
                Err(e) => cx.throw_error(e.message),
            }
        }
        Err(e) => cx.throw_error(e.to_string()),
    }
}
pub fn intersection(mut cx: FunctionContext) -> JsResult<JsValue> {
    let params = cx.argument::<JsObject>(0)?;

    let mut buf_lhs = String::new();
    let mut buf_rhs = String::new();
    let ttl_doc_lhs = make_doc(
        &params,
        &mut cx,
        &mut buf_lhs,
        PARAMS_LHS_PATH,
        PARAMS_LHS_DATA,
    );
    let ttl_doc_rhs = make_doc(
        &params,
        &mut cx,
        &mut buf_rhs,
        PARAMS_RHS_PATH,
        PARAMS_RHS_DATA,
    );
    fn intersection_fn<'a, 'b, C: Context<'a>>(
        params: &'b Handle<'b, JsObject>,
        cx: &mut C,
        lhs: TurtleDoc<'b>,
        rhs: TurtleDoc<'b>,
    ) -> JsResult<'a, JsValue> {
        let diff = lhs.intersection(&rhs);
        match diff {
            Ok(model) => make_response(params, cx, model),

            Err(e) => cx.throw_error(e.to_string()),
        }
    }
    match (ttl_doc_lhs, ttl_doc_rhs) {
        (Ok(DocType::TurtleDoc(lhs)), Ok(DocType::TurtleDoc(rhs))) => {
            intersection_fn(&params, &mut cx, lhs, rhs)
        }
        (Ok(DocType::TurtleDoc(lhs)), Ok(DocType::RdfJsonTriple((rhs, prefixes)))) => {
            match rdf_json_triple_to_doc(&rhs[..], prefixes) {
                Ok(doc) => intersection_fn(&params, &mut cx, lhs, doc),
                Err(e) => cx.throw_error(e.message),
            }
        }
        (Ok(DocType::RdfJsonTriple((lhs, prefixes))), Ok(DocType::TurtleDoc(rhs))) => {
            match rdf_json_triple_to_doc(&lhs[..], prefixes) {
                Ok(doc) => intersection_fn(&params, &mut cx, doc, rhs),
                Err(e) => cx.throw_error(e.message),
            }
        }
        (
            Ok(DocType::RdfJsonTriple((lhs, lhs_prefixes))),
            Ok(DocType::RdfJsonTriple((rhs, rhs_prefixes))),
        ) => {
            match (
                rdf_json_triple_to_doc(&lhs[..], lhs_prefixes),
                rdf_json_triple_to_doc(&rhs[..], rhs_prefixes),
            ) {
                (Ok(lhs), Ok(rhs)) => intersection_fn(&params, &mut cx, lhs, rhs),
                (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.message),
                (Err(e1), Err(e2)) => {
                    cx.throw_error(format!("error:\n-{}\n-{}", e1.message, e2.message))
                }
            }
        }
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.message),
        (Err(e1), Err(e2)) => cx.throw_error(format!("error:\n-{}\n-{}", e1.message, e2.message)),
    }
}

fn rdf_json_triple_to_doc(
    triples: &[RdfJsonTriple],
    prefixes: BTreeMap<String, String>,
) -> Result<TurtleDoc<'_>, TurtleDocError> {
    let stmts = Statement::from_rdf_json_triples(triples)?;
    let mut doc = TurtleDoc::try_from(stmts)?;
    doc.add_prefixes(prefixes);
    Ok(doc)
}

fn convert_rdf_json_triple_to_neon_object<'a, C: Context<'a>>(
    cx: &mut C,
    triple: RdfJsonTriple,
) -> JsResult<'a, JsObject> {
    let RdfJsonTriple {
        subject,
        predicate,
        object,
    } = triple;
    let stmt_obj = cx.empty_object();
    let subject = convert_rdf_json_node_result_to_neon_object(cx, subject)?;
    let predicate = convert_rdf_json_node_result_to_neon_object(cx, predicate)?;
    let object = convert_rdf_json_node_result_to_neon_object(cx, object)?;
    stmt_obj.set(cx, "subject", subject)?;
    stmt_obj.set(cx, "predicate", predicate)?;
    stmt_obj.set(cx, "object", object)?;
    Ok(stmt_obj)
}
fn convert_rdf_json_node_result_to_neon_object<'a, C: Context<'a>>(
    cx: &mut C,
    node_res: RdfJsonNodeResult,
) -> JsResult<'a, JsObject> {
    match node_res {
        RdfJsonNodeResult::SingleNode(node) => {
            let obj = cx.empty_object();
            let value = cx.string(node.value);
            let typ = cx.string(node.typ);
            obj.set(cx, "value", value)?;
            obj.set(cx, "type", typ)?;
            if let Some(lang) = node.lang {
                let lang = cx.string(lang);
                obj.set(cx, "lang", lang)?;
            }
            if let Some(dt) = node.datatype {
                let dt = cx.string(dt);
                obj.set(cx, "datatype", dt)?;
            }
            Ok(obj)
        }
        RdfJsonNodeResult::ListNodes(list) => {
            let array = JsArray::new(cx, list.len());
            for (idx, node) in list.into_iter().enumerate() {
                let obj = convert_rdf_json_node_result_to_neon_object(cx, node)?;
                array.set(cx, idx as u32, obj)?;
            }
            let object: Handle<JsObject> = array.upcast();

            Ok(object)
        }
    }
}

fn convert_neon_object_to_rdf_js_triple<'a, C: Context<'a>>(
    cx: &mut C,
    obj: Handle<JsObject>,
) -> Result<RdfJsonTriple, TurtleDocError> {
    let subject: Handle<JsObject> = obj.get(cx, "subject").map_err(|e| TurtleDocError {
        message: e.to_string(),
    })?;
    let predicate: Handle<JsObject> = obj.get(cx, "predicate").map_err(|e| TurtleDocError {
        message: e.to_string(),
    })?;
    let object: Handle<JsObject> = obj.get(cx, "object").map_err(|e| TurtleDocError {
        message: e.to_string(),
    })?;

    let subject = convert_neon_object_to_rdf_js_node_res(cx, subject)?;
    let predicate = convert_neon_object_to_rdf_js_node_res(cx, predicate)?;
    let object = convert_neon_object_to_rdf_js_node_res(cx, object)?;
    Ok(RdfJsonTriple {
        subject,
        predicate,
        object,
    })
}
fn convert_neon_object_to_rdf_js_node_res<'a, C: Context<'a>>(
    cx: &mut C,
    obj: Handle<JsObject>,
) -> Result<RdfJsonNodeResult, TurtleDocError> {
    if let Ok(array) = obj.downcast::<JsArray, _>(cx) {
        let js_arr: Vec<Handle<JsValue>> = array.to_vec(cx).map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;
        let mut arr = Vec::with_capacity(js_arr.len());
        for ja in js_arr {
            let ja: Handle<JsObject> =
                ja.downcast::<JsObject, _>(cx).map_err(|e| TurtleDocError {
                    message: e.to_string(),
                })?;
            arr.push(convert_neon_object_to_rdf_js_node_res(cx, ja)?);
        }
        Ok(RdfJsonNodeResult::ListNodes(arr))
    } else {
        let nod = convert_neon_object_to_rdf_js_node(cx, obj)?;
        Ok(RdfJsonNodeResult::SingleNode(nod))
    }
}
fn convert_neon_object_to_rdf_js_node<'a, C: Context<'a>>(
    cx: &mut C,
    obj: Handle<JsObject>,
) -> Result<RdfJsonNode, TurtleDocError> {
    let value: Handle<JsString> = obj.get(cx, "value").map_err(|e| TurtleDocError {
        message: e.to_string(),
    })?;
    let typ: Handle<JsString> = obj.get(cx, "type").map_err(|e| TurtleDocError {
        message: e.to_string(),
    })?;
    let datatype: Option<Handle<JsString>> =
        obj.get_opt(cx, "datatype").map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;

    let lang: Option<Handle<JsString>> = obj.get_opt(cx, "lang").map_err(|e| TurtleDocError {
        message: e.to_string(),
    })?;

    Ok(RdfJsonNode {
        typ: typ.value(cx),
        datatype: datatype.map(|dt| dt.value(cx)),
        lang: lang.map(|l| l.value(cx)),
        value: value.value(cx),
    })
}
fn make_response<'a, 'b, C: Context<'a>>(
    params: &'b Handle<'b, JsObject>,
    cx: &mut C,
    doc: TurtleDoc<'b>,
) -> JsResult<'a, JsValue> {
    // todo refactor this to offer more output type
    let as_n3: bool = params
        .get_opt::<JsString, _, _>(cx, PARAMS_OUTPUT_TYPE)
        .map(|ot| ot.map(|o| o.value(cx).as_str() == "n3").unwrap_or(false))?;
    let output_file_path: Option<String> = params
        .get_opt::<JsString, _, _>(cx, PARAMS_OUTPUT_FILE_PATH)
        .map(|otp| otp.map(|o| o.value(cx)))?;
    let buf_size: Option<usize> = params
        .get_opt::<JsNumber, _, _>(cx, PARAMS_BUF_SIZE)
        .map(|buf| buf.map(|b| b.value(cx).abs() as usize))?;

    let js_mapper_func = params.get_opt::<JsFunction, _, _>(cx, PARAMS_MAPPER_FUNCTION)?;

    if let Some(opf) = output_file_path {
        return match doc.to_file(opf, buf_size, !as_n3) {
            Ok(_) => {
                let b = cx.boolean(true);
                let b = b.as_value(cx);
                Ok(b)
            }
            Err(e) => cx.throw_error(e.to_string()),
        };
    } else if as_n3 {
        let ttl = doc.to_string();
        let s = cx.string(ttl);
        let s = s.as_value(cx);
        Ok(s)
    } else {
        let json_stmts: Vec<RdfJsonTriple> = (&doc).into();
        let array = JsArray::new(cx, json_stmts.len());
        let mut count = 0 as u32;
        for triple in json_stmts.into_iter() {
            let stmt_obj = convert_rdf_json_triple_to_neon_object(cx, triple)?;
            if let Some(fun) = js_mapper_func {
                let mapped_obj: Handle<JsValue> = fun.call_with(cx).arg(stmt_obj).apply(cx)?;
                if !mapped_obj.is_a::<JsNull, _>(cx) && !mapped_obj.is_a::<JsUndefined, _>(cx) {
                    array.set(cx, count, mapped_obj)?;
                    count += 1;
                }
            } else {
                array.set(cx, count, stmt_obj)?;
                count += 1;
            }
        }
        return Ok(array.upcast());
    }
}

fn make_doc<'a, 'b>(
    params: &'b Handle<'b, JsObject>,
    cx: &'b mut FunctionContext,
    buf: &'a mut String,
    key_path: &'static str,
    key_data: &'static str,
) -> Result<DocType<'a>, TurtleDocError> {
    let path: Option<Handle<JsString>> =
        params.get_opt(cx, key_path).map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;
    let data: Option<Handle<JsValue>> =
        params.get_opt(cx, key_data).map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;

    // well known prefix
    let well_known_prefix = if let Ok(Some(well_known_prefix)) =
        params.get_opt::<JsString, _, _>(cx, PARAMS_WELL_KNOWN_PREFIX)
    {
        let wkp = well_known_prefix.value(cx);
        Some(wkp)
    } else {
        None
    };

    // extract prefixes
    let prefixes: Option<Handle<JsObject>> =
        params
            .get_opt(cx, PARAMS_EXTRA_PREFIXES)
            .map_err(|e| TurtleDocError {
                message: e.to_string(),
            })?;

    let mut prefixes_map = BTreeMap::new();
    if let Some(prefixes) = prefixes {
        // do

        let properties = prefixes
            .get_own_property_names(cx)
            .and_then(|p| p.to_vec(cx))
            .map_err(|e| TurtleDocError {
                message: e.to_string(),
            })?;

        for property in properties {
            if let Ok(property) = property.downcast::<JsString, _>(cx) {
                let property = property.value(cx);
                let value: Result<Option<Handle<JsString>>, _> =
                    prefixes.get_opt(cx, property.as_str());
                if let Ok(Some(value)) = value {
                    let value = value.value(cx);
                    prefixes_map.insert(property, value);
                } else if let Err(e) = value {
                    eprintln!("warning! value for {property} is incorrect. {e}");
                }
            } else {
                eprintln!("warning! could not downcast property {property:?} to string");
            }
        }
    }
    if let Some(path) = path {
        let path = path.value(cx);
        match PathBuf::from(&path).extension().and_then(|s| s.to_str()) {
            Some("json") => {
                let triples = RdfJsonTriple::from_json_file(&path)?;
                Ok(DocType::RdfJsonTriple((triples, prefixes_map)))
            }
            _ => {
                let mut doc = TurtleDoc::from_file(path, well_known_prefix, buf)?;
                doc.add_prefixes(prefixes_map);
                Ok(DocType::TurtleDoc(doc))
            }
        }
    } else if let Some(data) = data {
        //convert_neon_object_to_rdf_js_triple
        if let Ok(data) = data.downcast::<JsString, _>(cx) {
            let data = data.value(cx);
            buf.push_str(&data);
            match TurtleDoc::try_from((buf.as_str(), well_known_prefix)) {
                Ok(mut doc) => {
                    doc.add_prefixes(prefixes_map);
                    Ok(DocType::TurtleDoc(doc))
                }
                Err(e) => match RdfJsonTriple::from_json(buf.as_str()) {
                    Ok(rjt) => Ok(DocType::RdfJsonTriple((rjt, prefixes_map))),
                    Err(e2) => Err(TurtleDocError {
                        message: format!("could not make doc from input:\n-{e}\n-{e2}"),
                    }),
                },
            }
        } else if let Ok(data) = data.downcast::<JsArray, _>(cx) {
            let js_arr = data.to_vec(cx).map_err(|e| TurtleDocError {
                message: e.to_string(),
            })?;
            let mut triples = Vec::with_capacity(js_arr.len());
            for ja in js_arr {
                let ja: Handle<JsObject> =
                    ja.downcast::<JsObject, _>(cx).map_err(|e| TurtleDocError {
                        message: e.to_string(),
                    })?;
                triples.push(convert_neon_object_to_rdf_js_triple(cx, ja)?)
            }
            Ok(DocType::RdfJsonTriple((triples, prefixes_map)))
        } else if let Ok(data) = data.downcast::<JsObject, _>(cx) {
            let triples = convert_neon_object_to_rdf_js_triple(cx, data)?;
            Ok(DocType::RdfJsonTriple((vec![triples], prefixes_map)))
        } else {
            return Err(TurtleDocError {
                message: "not implemented yet.".into(),
            });
        }
    } else {
        Err(TurtleDocError {
            message: format!("missing path ('{key_path}') or data ({key_data})"),
        })
    }
}
