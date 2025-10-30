use crate::c_actions::*;

pub fn count_loops(tu: &translation_unit) -> usize {
    tu.external_decls.iter().map(count_loops_external_decl).sum()
}

fn count_loops_external_decl(ed: &external_decl) -> usize {
    match ed {
        external_decl::C1(fd) => count_loops_function_definition(&fd.function),
        _ => 0,
    }
}

fn count_loops_function_definition(fd: &function_definition) -> usize {
    match fd {
        function_definition::C1(c) => count_loops_compound_stat(&c.body),
        function_definition::C2(c) => count_loops_compound_stat(&c.body),
        function_definition::C3(c) => count_loops_compound_stat(&c.body),
        function_definition::C4(c) => count_loops_compound_stat(&c.body),
    }
}

fn count_loops_compound_stat(cs: &compound_stat) -> usize {
    if let Some(items) = cs {
        items.iter().map(count_loops_block_item).sum()
    } else {
        0
    }
}

fn count_loops_block_item(bi: &block_item) -> usize {
    match bi {
        block_item::stat(s) => count_loops_stat(s),
        _ => 0,
    }
}

fn count_loops_stat(s: &stat) -> usize {
    match s {
        stat::compound_stat(cs) => count_loops_compound_stat(cs),
        stat::selection_stat(ss) => match ss {
            selection_stat::C1(c) => count_loops_stat(&c.stat),
            selection_stat::C2(c) => {
                count_loops_stat(&c.stat_5) + count_loops_stat(&c.stat_7)
            }
            selection_stat::C3(c) => count_loops_stat(&c.stat),
        },
        stat::iteration_stat(is) => {
            1 + match is {
                iteration_stat::C1(c) => count_loops_stat(&c.stat),
                iteration_stat::C2(c) => count_loops_stat(&c.stat),
                iteration_stat::C3(c) => count_loops_stat(&c.stat),
                iteration_stat::C4(c) => count_loops_stat(&c.stat),
            }
        }
        stat::labeled_stat(ls) => match ls {
            labeled_stat::C1(c) => count_loops_stat(&c.stat),
            labeled_stat::C2(c) => count_loops_stat(&c.stat),
            labeled_stat::stat(s) => count_loops_stat(s),
        },
        _ => 0,
    }
}
