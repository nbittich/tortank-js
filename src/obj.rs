use std::borrow::Cow;

use neon::prelude::*;
use tortank::turtle::turtle_doc::{
    Node, RdfJsonNodeResult, RdfJsonTriple, TurtleDoc, TurtleDocError,
};

pub fn difference(mut cx: FunctionContext) -> JsResult<JsObject> {
    let params = cx.argument::<JsObject>(0)?;

    let mut buf_lhs = String::new();
    let mut buf_rhs = String::new();
    let ttl_doc_lhs = make_doc(&params, &mut cx, &mut buf_lhs, "lhsPath", "lhsData");
    let ttl_doc_rhs = make_doc(&params, &mut cx, &mut buf_rhs, "rhsPath", "rhsData");

    match (ttl_doc_lhs, ttl_doc_rhs) {
        (Ok(lhs), Ok(rhs)) => {
            let diff = lhs.difference(&rhs);
            match diff {
                Ok(model) => {
                    let json_stmts: Vec<RdfJsonTriple> = (&model).into();
                    let array = JsArray::new(&mut cx, json_stmts.len() as u32);
                    for (idx, triple) in json_stmts.into_iter().enumerate() {
                        let stmt_obj = convert_rdf_json_triple_to_neon_object(&mut cx, triple)?;
                        array.set(&mut cx, idx as u32, stmt_obj)?;
                    }
                    Ok(array.upcast())
                }
                Err(e) => return cx.throw_error(e.to_string()),
            }
        }
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => return cx.throw_error(e.to_string()),
        (Err(e1), Err(e2)) => return cx.throw_error(e1.to_string() + &e2.to_string()),
    }
}

pub fn filter(mut cx: FunctionContext) -> JsResult<JsObject> {
    let params = cx.argument::<JsObject>(0)?;

    let mut buf = String::new();

    let subject: Option<Handle<JsString>> = params.get_opt(&mut cx, "subject")?;
    let predicate: Option<Handle<JsString>> = params.get_opt(&mut cx, "predicate")?;
    let object: Option<Handle<JsString>> = params.get_opt(&mut cx, "object")?;

    let ttl_doc = make_doc(&params, &mut cx, &mut buf, "ttlPath", "ttlData");

    match ttl_doc {
        Ok(ttl_doc) => {
            let subject = if let Some(subject) = subject {
                let subject = subject.value(&mut cx);
                Some(Node::Iri(Cow::Owned(subject)))
                // todo
            } else {
                None
            };
            let predicate = if let Some(predicate) = predicate {
                let predicate = predicate.value(&mut cx);
                Some(Node::Iri(Cow::Owned(predicate)))
                // todo
            } else {
                None
            };
            let object = if let Some(object) = object {
                let object = object.value(&mut cx);
                Some(Node::Iri(Cow::Owned(object)))
                // todo
            } else {
                None
            };
            let filtered_stmts = ttl_doc
                .list_statements(subject.as_ref(), predicate.as_ref(), object.as_ref())
                .into_iter()
                .map(|stmt| stmt.into())
                .collect::<Vec<RdfJsonTriple>>();
            let array = JsArray::new(&mut cx, filtered_stmts.len() as u32);
            for (idx, triple) in filtered_stmts.into_iter().enumerate() {
                let stmt_obj = convert_rdf_json_triple_to_neon_object(&mut cx, triple)?;
                array.set(&mut cx, idx as u32, stmt_obj)?;
            }
            Ok(array.upcast())
        }
        Err(e) => return cx.throw_error(e.to_string()),
    }
}
pub fn intersection(mut cx: FunctionContext) -> JsResult<JsObject> {
    let params = cx.argument::<JsObject>(0)?;

    let mut buf_lhs = String::new();
    let mut buf_rhs = String::new();
    let ttl_doc_lhs = make_doc(&params, &mut cx, &mut buf_lhs, "lhsPath", "lhsData");
    let ttl_doc_rhs = make_doc(&params, &mut cx, &mut buf_rhs, "rhsPath", "rhsData");

    match (ttl_doc_lhs, ttl_doc_rhs) {
        (Ok(lhs), Ok(rhs)) => {
            let diff = lhs.intersection(&rhs);
            match diff {
                Ok(model) => {
                    let json_stmts: Vec<RdfJsonTriple> = (&model).into();
                    let array = JsArray::new(&mut cx, json_stmts.len() as u32);
                    for (idx, triple) in json_stmts.into_iter().enumerate() {
                        let stmt_obj = convert_rdf_json_triple_to_neon_object(&mut cx, triple)?;
                        array.set(&mut cx, idx as u32, stmt_obj)?;
                    }
                    Ok(array.upcast())
                }
                Err(e) => return cx.throw_error(e.to_string()),
            }
        }
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => return cx.throw_error(e.to_string()),
        (Err(e1), Err(e2)) => return cx.throw_error(e1.to_string() + &e2.to_string()),
    }
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
            // todo
            let array = JsArray::new(cx, list.len() as u32);
            for (idx, node) in list.into_iter().enumerate() {
                let obj = convert_rdf_json_node_result_to_neon_object(cx, node)?;
                array.set(cx, idx as u32, obj)?;
            }
            let object: Handle<JsObject> = array.upcast();

            Ok(object)
        }
    }
}

fn make_doc<'a, 'b>(
    params: &'b Handle<'b, JsObject>,
    cx: &'b mut FunctionContext,
    buf: &'a mut String,
    key_path: &'static str,
    key_data: &'static str,
) -> Result<TurtleDoc<'a>, TurtleDocError> {
    let path: Option<Handle<JsString>> =
        params.get_opt(cx, key_path).map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;
    let data: Option<Handle<JsString>> =
        params.get_opt(cx, key_data).map_err(|e| TurtleDocError {
            message: e.to_string(),
        })?;

    if let Some(path) = path {
        TurtleDoc::from_file(path.value(cx), buf)
    } else if let Some(data) = data {
        buf.push_str(&data.value(cx));
        TurtleDoc::from_string(buf)
    } else {
        Err(TurtleDocError {
            message: format!("missing path ('{key_path}') or data ({key_data})"),
        })
    }
}
