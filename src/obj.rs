use neon::prelude::*;
use tortank::turtle::turtle_doc::{RdfJsonNodeResult, RdfJsonTriple, TurtleDoc, TurtleDocError};

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
        PARAMS_RHS_PATH,
    );

    match (ttl_doc_lhs, ttl_doc_rhs) {
        (Ok(lhs), Ok(rhs)) => {
            let combine = lhs + rhs;
            make_response(&params, &mut cx, combine)
        }
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.to_string()),
        (Err(e1), Err(e2)) => cx.throw_error(e1.to_string() + &e2.to_string()),
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
        PARAMS_RHS_PATH,
    );

    match (ttl_doc_lhs, ttl_doc_rhs) {
        (Ok(lhs), Ok(rhs)) => {
            let diff = lhs.difference(&rhs);
            match diff {
                Ok(model) => make_response(&params, &mut cx, model),

                Err(e) => cx.throw_error(e.to_string()),
            }
        }
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.to_string()),
        (Err(e1), Err(e2)) => cx.throw_error(e1.to_string() + &e2.to_string()),
    }
}

pub fn statements(mut cx: FunctionContext) -> JsResult<JsValue> {
    let params = cx.argument::<JsObject>(0)?;

    let mut buf = String::new();

    let subject: Option<Handle<JsString>> = params.get_opt(&mut cx, PARAMS_SUBJECT_NODE)?;
    let predicate: Option<Handle<JsString>> = params.get_opt(&mut cx, PARAMS_PREDICATE_NODE)?;
    let object: Option<Handle<JsString>> = params.get_opt(&mut cx, PARAMS_OBJECT_NODE)?;

    let ttl_doc = make_doc(&params, &mut cx, &mut buf, PARAMS_LHS_PATH, PARAMS_LHS_DATA);

    match ttl_doc {
        Ok(ttl_doc) => {
            let subject = subject.map(|subject| subject.value(&mut cx));
            let predicate = predicate.map(|predicate| predicate.value(&mut cx));
            let object = if let Some(object) = object {
                let object = object.value(&mut cx);
                Some(object)
            } else {
                None
            };
            let stmts_res = ttl_doc.parse_and_list_statements(subject, predicate, object);

            match stmts_res {
                Ok(stmts) => {
                    let filtered_stmts = stmts.into_iter().cloned().collect();
                    match TurtleDoc::from_statements(filtered_stmts) {
                        Ok(doc) => make_response(&params, &mut cx, doc),
                        Err(e) => cx.throw_error(e.to_string()),
                    }
                }
                Err(e) => cx.throw_error(e.to_string()),
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

    match (ttl_doc_lhs, ttl_doc_rhs) {
        (Ok(lhs), Ok(rhs)) => {
            let diff = lhs.intersection(&rhs);
            match diff {
                Ok(model) => make_response(&params, &mut cx, model),
                Err(e) => cx.throw_error(e.to_string()),
            }
        }
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => cx.throw_error(e.to_string()),
        (Err(e1), Err(e2)) => cx.throw_error(e1.to_string() + &e2.to_string()),
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

fn make_response<'a, 'b, C: Context<'a>>(
    params: &'b Handle<'b, JsObject>,
    cx: &mut C,
    doc: TurtleDoc<'b>,
) -> JsResult<'a, JsValue> {
    let out_type: Option<Handle<JsString>> = params.get_opt(cx, PARAMS_OUTPUT_TYPE)?;
    let output_file_path: Option<Handle<JsString>> = params.get_opt(cx, PARAMS_OUTPUT_FILE_PATH)?;
    let buf_size: Option<Handle<JsNumber>> = params.get_opt(cx, PARAMS_BUF_SIZE)?;

    // todo refactor this to offer more output type
    let as_n_3 = if let Some(out_type) = out_type {
        match out_type.value(cx).as_str() {
            "js" => false,
            "n3" => true,
            s => return cx.throw_error(format!("{s} is not a valid output type")),
        }
    } else {
        false
    };
    let output_file_path = output_file_path.map(|output_file_path| output_file_path.value(cx));
    let buf_size = buf_size.map(|buf_size| buf_size.value(cx).abs() as usize);

    if let Some(opf) = output_file_path {
        return match doc.to_file(opf, buf_size, !as_n_3) {
            Ok(_) => {
                let b = cx.boolean(true);
                let b = b.as_value(cx);
                Ok(b)
            }
            Err(e) => cx.throw_error(e.to_string()),
        };
    } else if as_n_3 {
        let ttl = doc.to_string();
        let s = cx.string(ttl);
        let s = s.as_value(cx);
        Ok(s)
    } else {
        let json_stmts: Vec<RdfJsonTriple> = (&doc).into();
        let array = JsArray::new(cx, json_stmts.len() as u32);
        for (idx, triple) in json_stmts.into_iter().enumerate() {
            let stmt_obj = convert_rdf_json_triple_to_neon_object(cx, triple)?;
            array.set(cx, idx as u32, stmt_obj)?;
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
