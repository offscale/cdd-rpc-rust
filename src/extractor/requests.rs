use cdd::*;

pub fn extract_from_ast(syntax: &syn::File) -> Result<Vec<Request>, failure::Error> {
    let mut visitor = crate::visitors::FunctionVisitor::new();
    syn::visit::visit_file(&mut visitor, &syntax);

    Ok(visitor
        .functions
        .into_iter()
        .map(|fnc| Request {
            name: fnc.name,
            params: Vec::new(),
            method: Method::Get,
            path: "/".to_string(),
            error_type: None,
            response_type: fnc.return_type.map(|rt| Box::new(rt)),
        })
        .collect())
}

pub fn struct_to_request(structure: (String, Vec<Variable>)) -> Option<Request> {
    let (name, _vars) = structure;

    // Some(Request {
    //     path: vars.iter().find(|&s| s.name == "path")?,
    //     error_type: vars.iter().find(|&s| s.name == "error_type")?,
    //     method: vars.iter().find(|&s| s.name == "error_type")?,
    //     response_type: vars.iter().find(|&s| s.name == "error_type")?,
    // })

    Some(Request {
        name,
        params: vec![],
        path: "".to_string(),
        error_type: None,
        method: Method::Get,
        response_type: None,
    })
}

// pub fn extract_requests(code: &str) -> Vec<Request> {
//     extract_structures(code)
//         .unwrap_or(HashMap::new())
//         .into_iter()
//         .map(struct_to_request)
//         .filter_map(|r| r)
//         .collect()
// }
