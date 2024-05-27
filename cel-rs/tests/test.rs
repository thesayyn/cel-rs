use cel_spec;

cel_spec::suite!(
    name = "basic",
    // TODO: fix these
    skip_section = "variables",
    skip_section = "functions",

    skip_test = "self_eval_ascii_escape_seq",
    skip_test = "self_eval_bytes_invalid_utf8",
    skip_test = "self_eval_unicode_escape_eight"
);

cel_spec::suite!(
    name = "comparisons",

    skip_section = "eq_wrapper",
    skip_section = "in_list_literal",
    skip_section = "in_map_literal",
    skip_section = "bound",
);
