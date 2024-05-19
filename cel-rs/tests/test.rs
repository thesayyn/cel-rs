use cel_spec;

cel_spec::suite!(
    name = "basic",
    include = "self_eval_zeroish",
    include = "self_eval_nonzeroish",
    include = "variables",
    // include = "ffunctions",
    // include = "reserved_const",
);

